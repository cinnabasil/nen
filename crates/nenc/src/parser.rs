use lexer::{Lexer, Token};

#[allow(dead_code)]
#[derive(Debug)]
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

    // Use up all tokens and print them
    pub fn token_drought(&mut self) {
        while let Some(token) = &self.current {
            println!("{:?}", token);
            self.current = self.lexer.next_token();
        }
    }
}
