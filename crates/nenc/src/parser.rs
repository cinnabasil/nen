use lexer::{Lexer, Token};

#[allow(dead_code)]
pub struct Parser {
    lexer: Lexer,
    current: Option<Token>
}

impl Parser {
    pub fn new(input: &str) -> Parser {
        let mut lexer = Lexer::new(input);
        let next_token = lexer.next_token();

        Parser {
            lexer,
            current: next_token
        }
    }
}
