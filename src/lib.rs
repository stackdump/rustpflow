extern crate wasm_bindgen;

use std::collections::HashMap;
use wasm_bindgen::prelude::*;

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

/*
NOTE: Place Definitions are purely informational
they are useful when describing the meaning of each value in the state machine
but are not needed to execute and validate state transformations
*/

/*
struct PlaceMap {
    schema: String,
    map: HashMap<String, i64>,
}
*/

struct Transition {
    role: String,
    delta: Vec<i64>,
    guards: HashMap<String, Vec<i64>>,
}

struct Machine {
    transitions: HashMap<String, Transition>,
}

impl Machine {
    fn action(&self, key: String) -> &Transition {
        return &self.transitions[&key];
    }
}

#[wasm_bindgen]
pub struct StateMachine {
    state: Vec<i64>,
    capacity: Vec<i64>,
    //places: PlaceMap,
    machine: Machine,
}

fn vadd(a: Vec<i64>, b: Vec<i64>) -> (Vec<i64>, bool) {
    let mut out: Vec<i64> = Vec::new();
    let mut ok: bool = true;
    let mut x: i64;

    for (aval, bval) in a.iter().zip(b) {
        x = aval + bval;

        if x < 0 {
            ok = false;
        }
        out.push(x);
    }

    return (out, ok);
}

#[wasm_bindgen]
impl StateMachine {
    pub fn get_state(&self) -> Vec<i64> {
        return self.state.clone();
    }

    // report required to execute an action
    pub fn get_role(&mut self, txn: String) -> String {
        return self.machine.action(txn).role.to_string();
    }

    pub fn transform(&mut self, txn: String) -> bool {
        let t = self.machine.action(txn);
        let (out, ok) = vadd(self.state.clone(), t.delta.clone());

        // test undercapacity
        if !ok {
            return false;
        }

        // test overcapacity
        for (i, cap) in self.capacity.iter().enumerate() {
            if *cap > 0 && out[i] > *cap {
                return false;
            }
        }

        // test guard conditions
        for (_, condition) in &t.guards {
            let (_, fail) = vadd(self.state.clone(), condition.clone());
            if fail {
                return false;
            }
        }

        self.state = out;
        return true;
    }
}

// construct a counter state machine
#[wasm_bindgen]
pub fn counter_machine() -> StateMachine {
    StateMachine {
        state: vec![0, 0, 0], // initial state
        capacity: vec![0, 0, 1],
        /*
        places: PlaceMap {
            schema: "counter".to_string(),
            map: map! {
                "p0".to_string() => 0,
                "p1".to_string() => 1,
                "p2".to_string() => 2
            },
        },
        */
        machine: Machine {
            transitions: map! {
                "inc0".to_string() => Transition {
                    role: "default".to_string(),
                    delta: vec![1, 0, 0],
                    guards: map! {
                        "atomic".to_string() => vec![-1, 0, 0]
                    },
                },
                "inc1".to_string() => Transition {
                    role: "default".to_string(),
                    delta: vec![0, 1, 0],
                    guards: HashMap::new(),
                },
                "inc2".to_string() => Transition {
                    role: "default".to_string(),
                    delta: vec![0, 0, 1],
                    guards: HashMap::new(),
                },
                "dec0".to_string() => Transition {
                    role: "default".to_string(),
                    delta: vec![-1, 0, 0],
                    guards: HashMap::new(),
                },
                "dec1".to_string() => Transition {
                    role: "default".to_string(),
                    delta: vec![0, -1, 0],
                    guards: HashMap::new(),
                },
                "dec2".to_string() => Transition {
                    role: "default".to_string(),
                    delta: vec![0, 0, -1],
                    guards: HashMap::new(),
                }
            },
        },
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_fields() {
        let mut sm = counter_machine();
        assert_eq!(sm.get_role("inc0".to_string()), "default");
        assert_eq!(sm.get_state()[0], 0);
    }

    #[test]
    fn test_guard_condition() {
        let mut sm = counter_machine();
        assert!(sm.transform("inc0".to_string()));
        assert_eq!(sm.get_state(), vec![1, 0, 0]);

        assert!(!sm.transform("inc0".to_string())); // fails guard condition
        assert_eq!(sm.get_state(), vec![1, 0, 0]);
    }

    #[test]
    fn test_valid_transform() {
        let mut sm = counter_machine();
        assert_eq!(sm.state, vec![0, 0, 0]);
        assert!(sm.transform("inc1".to_string()));
        assert!(sm.transform("inc1".to_string()));
        assert_eq!(sm.get_state(), vec![0, 2, 0]); // valid actions work
    }

    #[test]
    fn test_capacity_limit() {
        let mut sm = counter_machine();
        assert!(!sm.transform("dec0".to_string())); // fails undercapacity check

        assert!(sm.transform("inc2".to_string()));
        assert_eq!(sm.get_state(), vec![0, 0, 1]);

        assert!(!sm.transform("inc2".to_string())); // fails overcapacity check
        assert_eq!(sm.get_state(), vec![0, 0, 1]);
    }
}
