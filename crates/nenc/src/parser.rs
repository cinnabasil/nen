use std::mem::discriminant;

use lexer::{Lexer, Token};

#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
    current: Option<Token>
}

#[derive(Debug)]
pub enum Expression {
    FunctionCall(String, Vec<Expression>)
}

#[derive(Debug)]
pub enum Statement {

}

#[derive(Debug)]
pub enum Node {
    Expression(Expression),
    Statement(Statement)
}

impl Parser {
    pub fn new(input: &str) -> Parser {
        let lexer = Lexer::new(input);

        Parser {
            lexer,
            current: None
        }
    }

    #[inline(always)]
    fn expect_token(&mut self, token: Token, consume: bool) {
        let next = if consume { self.lexer.next_token() } else { self.lexer.peek_token() };
        let mut is_correct = false;
        if next.is_some() {
            if discriminant(&next.unwrap()) == discriminant(&token) {
                is_correct = true; 
            }
        }

        if !is_correct {
            todo!("Handle unexpected token");
        }
    }

    #[allow(dead_code)]
    pub fn next_node(&mut self) -> Option<Node> {
        self.current = self.lexer.next_token(); 
        match &self.current {
            Some(Token::Identifier(c)) => {
                match self.lexer.peek_token() {
                    Some(Token::OpenParen) => {
                        // Function Call
                        // Don't handle args yet
                        let function_name = c.to_string();
                        self.expect_token(Token::OpenParen, true);
                        self.expect_token(Token::CloseParen, true);
                        self.expect_token(Token::Semicolon, true);

                        Some(
                            Node::Expression(
                                Expression::FunctionCall(
                                    function_name,
                                    Vec::<Expression>::new()
                                )
                            )
                        )
                    },
                    Some(t) => {
                        todo!("Don't know how to handle token: {t:?}");
                    },
                    None => {
                        todo!("Identifier not followed by (");
                    }
                }
            },
            Some(t) => {
                todo!("Don't know how to handle token: {t:?}");
            },
            None => None
        }
    }

    // Use up all tokens and print them
    pub fn token_drought(&mut self) {
        while let Some(token) = self.lexer.next_token() {
            println!("{:?}", token);
        }
    }
}
