import os, sys, imp
import xml.etree.ElementTree as ET

class InvalidFormat(Exception):
    pass

class PFlowParser(object):

    def _parse(self, path):
        self.root = ET.parse(path).getroot()
        self.nets = self.root.findall('subnet')
        self._subnet(self.nets[0])

    def _subnet(self, el):
        for net in el.findall('subnet'):
            self.nets.append(net)
            self._subnet(net)

    def _pflow(self):
        return {
            "roles": self._roles(),
            "arcs": self._arcs(),
            "places": self._places(),
            "ref_places": self._ref_places(),
            "transitions": self._transitions()
        }

    @staticmethod
    def _el_to_dict(el):
        d = {}
        for c in el:
            if c.tag in ("label", "type", "name"):
                d[c.tag] = c.text
            elif c.tag in ("isStatic", "createCase", "destroyCase"):
                d[c.tag] = (c.text == "true")
            elif c.tag in ("transitionId"):
                if "transitionId" not in d:
                    d["transitionId"] = []
                d["transitionId"].append(int(c.text))
            else:
                d[c.tag] = int(c.text)

        return d

    def _roles(self):
        rl = []
        for r in self.root.findall('roles'):
            rl = rl + r.findall('role')

        return [ self._el_to_dict(el) for el in rl ]

    def _arcs(self):
        ar = []
        for n in self.nets:
            ar = ar + n.findall('arc')

        return [ self._el_to_dict(el) for el in ar ]

    def _places(self):
        pl = []
        for n in self.nets:
            pl = pl + n.findall('place')

        return sorted([ self._el_to_dict(el) for el in pl ], key=lambda x: x['id'])

    def _ref_places(self):
        rp = []
        for n in self.nets:
            rp = rp + n.findall('referencePlace')

        return [ self._el_to_dict(el) for el in rp ]

    def _transitions(self):
        tx = []
        for n in self.nets:
            tx = tx + n.findall('transition')

        return sorted([ self._el_to_dict(el) for el in tx ], key=lambda x: x['id'])

class PFlowNet(PFlowParser):

    def __init__(self, path):
        self.name = os.path.splitext(os.path.basename(path))[0]
        self._parse(path)
        self._reindex(self._pflow())

    def empty_vector(self):
        return [0 for _ in self.place_ids]

    def _reindex(self, pflow):
        # REVIEW: isStatic attribute indicates that a place is shared
        # between subnets - though in the VASS form this is the default behavior

        i = 0
        self.place_ids = {}
        self.place_labels = []
        self.places = {}

        for p in pflow['places']:
            self.place_ids[p['id']] = i

            # capacity tag unsupported by pneditor.org
            # latest/0.71 version of pflow
            # though this can be added manually to the file after initial design using GUI
            self.place_labels.append(p['label'])
            if 'capacity' in p:
                cap = p['capacity']
            else:
                cap = 0 

            self.places[p['label']] = {
                "offset": i,
                "capacity": cap,
                "initial": p['tokens']
            }

            i+=1

        self.ref_places = {}
        for rp in pflow['ref_places']:
            self.ref_places[rp['id']] = rp['connectedPlaceId']

        self.transitions = {}
        self.transition_ids = {}

        for t in pflow['transitions']:
            self.transition_ids[t['id']] = t['label']
            self.transitions[t['label']] = {
                "delta": self.empty_vector(),
                "role": "default",
                "guards": {}
            }

        for r in pflow['roles']:
            for t in r['transitionId']:
                key = self.transition_ids[t]
                self.transitions[key]['role'] = r['name']


        for a in pflow['arcs']:
            p = None
            t = None

            if a['destinationId'] in self.transition_ids:
                tx_key = 'destinationId'
                pl_key = 'sourceId'
                unit = -1 * a['multiplicity']
            elif a['sourceId'] in self.transition_ids:
                tx_key = 'sourceId'
                pl_key = 'destinationId'
                unit = 1 * a['multiplicity']
            else:
                assert False

            t = self.transition_ids[a[tx_key]]

            try:
                p = self.place_ids[a[pl_key]]
            except KeyError:
                p = self.place_ids[self.ref_places[a[pl_key]]]

            pl = self.places[self.place_labels[p]]
            tx = self.transitions[t]
                
            if a['type'] == 'inhibitor':
                g = self.empty_vector()
                g[pl['offset']] = unit
                tx['guards'][self.place_labels[p]] = g
            elif a['type'] == 'regular':
                tx['delta'][pl['offset']] = unit
            else:
                assert False

# REVIEW: consider relocating
class StateMachine(object):

    storage_provider = None
    """ storage provider class """

    def __init__(self, pflow):
        self.name = pflow.name
        self.places = pflow.places
        self.transitions = pflow.transitions

    def __str__(self):
        """ print source code """
        return self.to_source()

    def to_source(self):
        """ generate state machine source code """

        # REVIEW: could we infer source modules via introspection?
        out = ""
        out += "\nfrom ptflow.state import StateMachine\n\n\n"
        out += "class Machine(StateMachine, Storage):\n\n"

        out += "    places = {\n"
        for t, attrib in self.places.items():
            out += "        '%s': %s,\n" % (t, attrib)
        out += "    }\n\n"

        out += "    transitions = {\n"
        for t, attrib in self.transitions.items():
            out += \
            "        '%s': {\n" % t + \
            "            'delta': %s,\n" % attrib['delta'] + \
            "            'role': '%s',\n" % attrib  ['role']

            if len(attrib['guards']) > 0:
                for l, g in attrib['guards'].items():
                    out += \
                    "            'guards': {\n" + \
                    "                '%s': %s,\n" % (l, g)
                out += "            }\n"
            else:
                out += "            'guards': {},\n" \

            out += "        },\n"

        out += "    }\n"

        return out
