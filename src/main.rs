mod helper;
use helper::parser;

mod nfa;
use nfa::NFA;

fn main() {
    let regex = "(a|b)*abb";
    let tokens = parser::tokenize(regex);
    println!("Tokens: {:?}", tokens);

    let postfix = parser::to_postfix(tokens);
    println!("Postfix: {:?}", postfix);

    let nfa = NFA::from_char('a');
    println!("NFA: {:?}", nfa);

    let nfa = nfa.rename_states(1);
    println!("NFA: {:?}", nfa);
}
