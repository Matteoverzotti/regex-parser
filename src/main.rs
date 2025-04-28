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

    let nfa: NFA = NFA::from_char('\0');
    nfa.visualize();

    // nfa = NFA::from_char('y');
    // nfa = nfa.rename_states(2);

    // nfa.visualize();


}
