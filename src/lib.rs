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

#[wasm_bindgen]
pub fn new() -> StateMachine {
    StateMachine {
        state: vec![0, 0, 1],
        capacity: vec![0, 0, 0],
        machine: Machine {
            transitions: map! {
                "INC0".to_string() => Transition {
                    delta: vec![1, 0, 0],
                    role: "role0".to_string(),
                    guards: map! {
                        "CHECK".to_string() => vec![0, 0, -1]
                    }
                },
                "INC1".to_string() => Transition {
                    delta: vec![0, 1, 0],
                    role: "role1".to_string(),
                    guards: map! {
                        "CHECK".to_string() => vec![0, 0, -1]
                    }
                },
                "DEC0".to_string() => Transition {
                    delta: vec![-1, 0, 0],
                    role: "role0".to_string(),
                    guards: HashMap::new(),
                },
                "DEC1".to_string() => Transition {
                    delta: vec![0, -1, 0],
                    role: "role1".to_string(),
                    guards: HashMap::new(),
                },
                "OFF".to_string() => Transition {
                    delta: vec![0, 0, 1],
                    role: "default".to_string(),
                    guards: HashMap::new(),
                },
                "ON".to_string() => Transition {
                    delta: vec![0, 0, -1],
                    role: "default".to_string(),
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
        let mut sm = new();
        assert_eq!(sm.get_role("INC0".to_string()), "role0");

        assert!(!sm.transform("INC0".to_string())); // fails due to guard
        assert_eq!(sm.get_state(), [0, 0, 1]);

        assert!(sm.transform("ON".to_string())); // enabled
        assert_eq!(sm.get_state(), [0, 0, 0]); //

        assert!(sm.transform("INC0".to_string())); // passes
        assert_eq!(sm.get_state(), [1, 0, 0]);

        assert!(!sm.transform("DEC1".to_string())); // fails due to underflow
        assert_eq!(sm.get_state(), [1, 0, 0]);
    }
}
