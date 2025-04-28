use crate::nfa::NFA;
use std::{collections::HashMap, fmt::format};

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

pub fn union(mut nfa1: NFA, mut nfa2: NFA) -> NFA {
    let total_states = nfa1.states.len() + nfa2.states.len();
    let start = String::from("q0");
    let end = format!("q{}", total_states + 1);

    nfa1 = nfa1.rename_states(1);
    nfa2 = nfa2.rename_states(nfa1.states.len() + 1);

    let mut transitions: HashMap<String, HashMap<char, Vec<String>>> = HashMap::new();
    transitions.insert(start.clone(), {
        let mut map = HashMap::new();
        map.insert('\0', vec![nfa1.q0, nfa2.q0]);
        map
    });

    for (state, map) in nfa1.transitions.into_iter().chain(nfa2.transitions.into_iter()) {
        transitions.insert(state, map);
    }

    for final_state in nfa1.final_states {
        transitions.entry(final_state).or_insert_with(HashMap::new)
            .entry('\0').or_insert_with(Vec::new)
            .push(end.clone());
    }

    for final_state in nfa2.final_states {
        transitions.entry(final_state).or_insert_with(HashMap::new)
            .entry('\0').or_insert_with(Vec::new)
            .push(end.clone());
    }

    let mut states = vec![start.clone(), end.clone()];
    states.extend(nfa1.states);
    states.extend(nfa2.states);

    let mut sigma = nfa1.sigma;
    sigma.extend(nfa2.sigma);
    sigma.sort();
    sigma.dedup();

    NFA {
        sigma,
        states,
        q0: start,
        final_states: vec![end],
        transitions,
    }
}

pub fn star(mut nfa: NFA) -> NFA {
    let start = String::from("q0");
    let end = format!("q{}", nfa.states.len() + 1);

    nfa = nfa.rename_states(1);

    let mut transitions = HashMap::new();
    transitions.insert(start.clone(), {
        let mut map = HashMap::new();
        map.insert('\0', vec![nfa.q0.clone(), end.clone()]);
        map
    });

    for (state, map) in nfa.transitions {
        transitions.insert(state, map);
    }

    for final_state in nfa.final_states {
        transitions.entry(final_state).or_insert_with(HashMap::new)
            .entry('\0').or_insert_with(Vec::new)
            .extend(vec![nfa.q0.clone(), end.clone()]);
    }

    let mut states = vec![start.clone(), end.clone()];
    states.extend(nfa.states);

    let sigma = nfa.sigma;

    NFA {
        sigma,
        states,
        q0: start,
        final_states: vec![end],
        transitions,
    }
}