use std::collections::HashMap;

// macro to construct hashes
macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

struct PlaceMap {
    schema: String,
    map: HashMap<String, i64>
}

// TODO: add guards
struct Transition {
    role: String,
    delta: Vec<i64>,
}

struct Machine {
    transitions: HashMap<String, Transition>,
}

impl Machine {
    fn action(&mut self, key: String) -> Vec<i64> {
        let mut out: Vec<i64> = Vec::new();

        // REVIEW: is there another way to clone?
        for val in self.transitions[&key].delta.iter() { 
            out.push(*val);
        }
        out
    }
}

struct StateMachine {
    state: Vec<i64>,
    capacity: Vec<i64>,
    places: PlaceMap,
    machine: Machine,
}

impl StateMachine {

    fn transform(&mut self, txn: String) {
        let mut out: Vec<i64> = Vec::new();
        let mut x: i64;

        // REVIEW: could action be used w/o copying Delta?
        for (aval, bval) in self.state.iter().zip(self.machine.action(txn)) {
            x = aval + bval;
            // TODO: make other assertions for guards, roles & overcapacity
            assert!(x >= 0); // assert not under capacity
            out.push(x);
        }
        self.state = out;
    }

}

// construct a counter state machine
fn counter_machine() -> StateMachine {

    StateMachine {
        state: vec![0,0,0],
        capacity: vec![0,0,0],
        places: PlaceMap {
            schema: "counter".to_string(),
            map: map!{
                "p0".to_string() => 0,
                "p1".to_string() => 1,
                "p2".to_string() => 2
            }
        },
        machine: Machine {
            transitions: map!{
                "inc0".to_string() => Transition {
                    role: "default".to_string(),
                    delta: vec![1, 0, 0]
                },
                "inc1".to_string() => Transition {
                    role: "default".to_string(),
                    delta: vec![0, 1, 0]
                },
                "inc2".to_string() => Transition {
                    role: "default".to_string(),
                    delta: vec![0, 0, 1]
                },
                "dec0".to_string() => Transition {
                    role: "default".to_string(),
                    delta: vec![-1, 0, 0]
                },
                "dec1".to_string() => Transition {
                    role: "default".to_string(),
                    delta: vec![0, -1, 0]
                },
                "dec2".to_string() => Transition {
                    role: "default".to_string(),
                    delta: vec![0, 0, -1]
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_fields() {
        let sm = counter_machine();
        let m = sm.machine;
        let p = sm.places;

        assert_eq!(p.schema, "counter");
        assert_eq!(p.map["p0"], 0);
        assert_eq!(sm.state[0], 0);
        assert_eq!(m.transitions["inc0"].role, "default");
        assert_eq!(m.transitions["inc0"].delta, vec![1, 0, 0]);
    }

    #[should_panic]
    #[test]
    fn test_invalid_transform() {
        let mut sm = counter_machine();
        assert_eq!(sm.state, vec![0, 0, 0]);
        sm.transform("dec0".to_string());
    }

    #[test]
    fn test_valid_transform() {
        let mut sm = counter_machine();
        assert_eq!(sm.state, vec![0, 0, 0]);

        sm.transform("inc0".to_string());
        assert_eq!(sm.state, vec![1, 0, 0]);
    }
}
