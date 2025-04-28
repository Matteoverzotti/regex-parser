mod helper;
use helper::parser;

mod nfa;
mod dfa;

fn main() {
    let regex = "a(bc)*d(e|f(g|h))*";
    let tokens = parser::tokenize(regex);
    println!("Tokens: {:?}", tokens);

    let postfix = parser::to_postfix(tokens);
    println!("Postfix: {:?}", postfix);

    let nfa = parser::build_nfa(postfix);
    // nfa.visualize();

    let dfa = nfa.to_dfa();
    println!("DFA: {:?}", dfa);
    // dfa.visualize();

    println!("{}", dfa.accepts_word("adfg"))
}
