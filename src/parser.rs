#[derive(Debug, Clone, Copy)]
pub enum Token {
    Char(char),
    Union,      // |
    Star,       // *
    Plus,       // +
    Question,   // ?
    Concat,     // explicit concatenation
    LeftParen,  // (
    RightParen, // )
}

pub fn tokenize(regex: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut prev_was_char = false;

    for c in regex.chars() {
        let token = match c {
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '*' => Token::Star,
            '+' => Token::Plus,
            '?' => Token::Question,
            '|' => Token::Union,
            _ => Token::Char(c),
        };

        // Handle explicit concatenation
        // If the previous token was a character or a right parenthesis,
        // and the current token is a character or a left parenthesis,
        // we need to insert an explicit concatenation token
        if prev_was_char {
            match token {
                Token::Char(_) | Token::LeftParen => {
                    tokens.push(Token::Concat);
                }
                _ => {}
            }
        }

        tokens.push(token);

        prev_was_char = matches!(token, Token::Char(_) | Token::RightParen | Token::Star | Token::Plus | Token::Question);
    }

    tokens
}
