use crate::nfa::NFA;
use std::collections::HashMap;

pub fn concat(nfa1: NFA, mut nfa2: NFA) -> NFA {
    let mut transitions = nfa1.transitions.clone();
    
    nfa2 = nfa2.rename_states(nfa1.states.len());

    // Add epsilon transitions from the final states of nfa1 to the initial state of nfa2
    for final_state in nfa1.final_states {
        transitions
            .entry(final_state).or_insert_with(HashMap::new)
            .entry('\0').or_insert_with(Vec::new)
            .push(nfa2.q0.clone());
    }

    for (state, map) in nfa2.transitions {
        transitions.insert(state, map);
    }

    let mut states = nfa1.states.clone();
    states.extend(nfa2.states.clone());

    let mut sigma = nfa1.sigma.clone();
    sigma.extend(nfa2.sigma.clone());
    sigma.sort();
    sigma.dedup();


    NFA::new(
        sigma,
        states,
        nfa1.q0,
        nfa2.final_states,
        transitions
    )
}