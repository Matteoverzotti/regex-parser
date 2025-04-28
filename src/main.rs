mod parser;

fn main() {
    let regex = "(a|b)*abb";
    let tokens = parser::tokenize(regex);
    println!("{:?}", tokens);
}
