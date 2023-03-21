#[allow(dead_code)]
pub struct Lexer {
    input: String,
    index: usize
}

pub enum Token {

}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input: input.to_string(),
            index: 0
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        None
    }
}
