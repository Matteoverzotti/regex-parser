use std::collections::{HashMap, HashSet, VecDeque};
use crate::nfa::NFA;

#[derive(Debug, Clone)]
pub struct DFA {
    pub sigma: Vec<char>,
    pub states: Vec<Vec<String>>,
    pub q0: Vec<String>,
    pub final_states: Vec<Vec<String>>,
    pub transitions: HashMap<Vec<String>, HashMap<char, Vec<String>>>,
}

impl NFA {
    fn epsilon_closure(&self, states: &HashSet<String>) -> HashSet<String> {
        let mut closure: HashSet<String> = states.clone();
        let mut stack: Vec<String> = states.iter().cloned().collect();

        while let Some(state) = stack.pop() {
            if let Some(transitions) = self.transitions.get(&state) {
                if let Some(next_states) = transitions.get(&'\0') {
                    for next_state in next_states {
                        if closure.insert(next_state.clone()) {
                            stack.push(next_state.clone());
                        }
                    }
                }
            }
        }

        closure
    }

    fn move_via_symbol(&self, states: &HashSet<String>, symbol: char) -> HashSet<String> {
        let mut next_states: HashSet<String> = HashSet::new();

        for state in states {
            if let Some(transitions) = self.transitions.get(state) {
                if let Some(target_states) = transitions.get(&symbol) {
                    for target_state in target_states {
                        next_states.insert(target_state.clone());
                    }
                }
            }
        }

        next_states
    }

    fn convert_set_to_state(&self, states: &HashSet<String>) -> Vec<String> {
        let mut state_vec: Vec<String> = states.iter().cloned().collect();
        state_vec.sort();
        state_vec
    }

    pub fn to_dfa(&self) -> DFA {
        let sigma: Vec<char> = self.sigma.iter().cloned()
            .filter(|&c| c != '\0')
            .collect();

        let mut init: HashSet<String> = HashSet::new();
        init.insert(self.q0.clone());

        let start_closure: HashSet<String> = self.epsilon_closure(&init);
        let start_state: Vec<String> = self.convert_set_to_state(&start_closure);

        let mut states: Vec<Vec<String>> = Vec::new();
        let mut transitions: HashMap<Vec<String>, HashMap<char, Vec<String>>> = HashMap::new();

        let mut queue: VecDeque<Vec<String>> = VecDeque::new();
        queue.push_back(start_state.clone());

        while let Some(current_state) = queue.pop_front() {
            if !states.contains(&current_state) {
                states.push(current_state.clone());
            }

            let current_set: HashSet<String> = current_state.iter().cloned().collect();
            let mut state_transitions: HashMap<char, Vec<String>> = HashMap::new();

            for &symbol in &sigma {
                let next_set: HashSet<String> = self.move_via_symbol(&current_set, symbol);
                let next_closure: HashSet<String> = self.epsilon_closure(&next_set);
                let next_state: Vec<String> = self.convert_set_to_state(&next_closure);

                if !states.contains(&next_state) && !queue.contains(&next_state) {
                    queue.push_back(next_state.clone());
                }

                state_transitions.insert(symbol, next_state.clone());
            }

            transitions.insert(current_state.clone(), state_transitions);
        }

        let mut final_states: Vec<Vec<String>> = Vec::new();
        for state in &states {
            if state.iter().any(|s| self.final_states.contains(s)) {
                final_states.push(state.clone());
            }
        }

        DFA {
            sigma,
            states,
            q0: start_state,
            final_states,
            transitions,
        }
    }
}

// impl DFA {
//     pub fn check_validity(&self) -> bool {
//         // I know that theoretically, the DFA should have at most one initial state
//         // So q0 should just be a &str
//         // But that made the validation logic more complicated
//         // So I'm just going to keep it as a Vec<&str> for now
//         if self.0.q0.len() != 1 {
//             return false;
//         }

//         for (state, transitions) in self.transitions.iter() {
//             if !self.states.contains(state) {
//                 return false;
//             }

//             for (symbol, next_state) in transitions {
//                 // This check isn't in the requirements but I added it anyways
//                 // Bonus points? :D
//                 if next_state.len() != 1 {
//                     return false;
//                 }

//                 if !self.sigma.contains(&symbol) || !self.states.contains(&next_state[0]) {
//                     return false;
//                 }
//             }
//         }
//         true
//     }

//     pub fn accepts_word(&self, word: &str) -> bool {
//         let mut current_state = self.q0[0];
//         for symbol in word.chars() {
//             match self.transitions.get(current_state)
//                 .and_then(|map| map.get(&symbol)) {
//                 None => return false,
//                 Some(next_state) => current_state = &next_state[0],
//             };
//         }

//         self.final_states.contains(&current_state)
//     }
// }
