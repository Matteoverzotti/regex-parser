mod helper;
use helper::parser;

mod nfa;

fn main() {
    let regex = "a(a+b)*b";
    let tokens = parser::tokenize(regex);
    println!("Tokens: {:?}", tokens);

    let postfix = parser::to_postfix(tokens);
    println!("Postfix: {:?}", postfix);

    let nfa = parser::build_nfa(postfix);
    nfa.visualize();
}
