mod parser;

fn main() {
    let regex = "(a|b)*abb";
    let tokens = parser::tokenize(regex);
    println!("Tokens: {:?}", tokens);

    let postfix = parser::to_postfix(tokens);
    println!("Postfix: {:?}", postfix);
}
