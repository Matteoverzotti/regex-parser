use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct NFA {
    pub(crate) sigma: Vec<char>,
    pub(crate) states: Vec<String>,
    pub(crate) q0: String,
    pub(crate) final_states: Vec<String>,
    pub(crate) transitions: HashMap<String, HashMap<char, Vec<String>>>
}

impl NFA {
    pub fn new(sigma: Vec<char>, states: Vec<String>, q0: String, final_states: Vec<String>, transitions: HashMap<String, HashMap<char, Vec<String>>>) -> NFA {
        NFA {
            sigma,
            states,
            q0,
            final_states,
            transitions
        }
    }

    pub fn from_char(c: char) -> NFA {
        let mut transitions = HashMap::new();
        transitions.insert("q0".to_string(), HashMap::new());
        transitions.get_mut("q0").unwrap().insert(c, vec!["q1".to_string()]);
        NFA::new(vec![c], vec!["q0".to_string(), "q1".to_string()], "q0".to_string(), vec!["q1".to_string()], transitions)
    }

    // Renames the states of the NFA to q{start_index}, q{start_index + 1}, ... and so on
    // This is useful when we want to combine multiple NFAs into one
    pub fn rename_states(self, start_index: usize) -> NFA {
        let mut mapping: HashMap<String, String> = HashMap::new();
        let mut new_idx: usize = start_index;

        for state in &self.states {
            mapping.insert(state.clone(), format!("q{}", new_idx));
            new_idx += 1;
        }

        let mut new_transitions: HashMap<String, HashMap<char, Vec<String>>> = HashMap::new();
        for (old_from, map) in self.transitions.into_iter() {
            let new_from: String = mapping[&old_from].clone();
            let entry: &mut HashMap<char, Vec<String>> = new_transitions.entry(new_from.clone()).or_insert_with(HashMap::new);

            for (symbol, to_list) in map {
                let targets: Vec<String> = to_list.into_iter().map(|old_to| mapping[&old_to].clone()).collect::<Vec<_>>();

                entry.entry(symbol).or_insert_with(Vec::new).extend(targets);
            }
        }

        let new_states: Vec<String> = mapping.values().cloned().collect();
        let new_q0: String = mapping[&self.q0].clone();
        let new_finals: Vec<String> = self
            .final_states
            .into_iter()
            .map(|old_f: String| mapping[&old_f].clone())
            .collect();

        NFA {
            sigma: self.sigma,
            states: new_states,
            q0: new_q0,
            final_states: new_finals,
            transitions: new_transitions,
        }
    }
}