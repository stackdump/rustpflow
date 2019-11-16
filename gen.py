#!/usr/bin/env python

import unittest
import pflow

if __name__ == '__main__':
    p = pflow.PFlowNet('./src/counter.pflow')
    sm = pflow.StateMachine(p)
    print(sm.to_source())
