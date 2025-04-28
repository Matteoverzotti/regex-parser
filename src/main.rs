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

    let nfa1: NFA = NFA::from_char('a');
    nfa1.visualize();

    let nfa2 = NFA::from_char('b');
    nfa2.visualize();

    let nfa3 = thompson::union(nfa1, nfa2);
    nfa3.visualize();

    let nfa4 = thompson::star(nfa3);
    nfa4.visualize();
    
}
