mod helper;
use helper::{parser, thompson};

mod nfa;
use nfa::NFA;

fn main() {
    let regex = "(a|b)*abb";
    let tokens = parser::tokenize(regex);
    println!("Tokens: {:?}", tokens);

    let postfix = parser::to_postfix(tokens);
    println!("Postfix: {:?}", postfix);

    let nfa1: NFA = NFA::from_char('\0');
    nfa1.visualize();

    let nfa2 = NFA::from_char('y');
    nfa2.visualize();

    let nfa3 = thompson::concat(nfa1, nfa2);
    let nfa4 = thompson::concat(nfa3.clone(), nfa3.clone());
    nfa4.visualize();
}
