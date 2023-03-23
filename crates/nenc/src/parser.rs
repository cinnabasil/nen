use std::mem::discriminant;

use lexer::{Lexer, Token, Keyword};

#[derive(Debug)]
pub struct Parser {
    lexer: Lexer
}


pub type Program = Vec<Node>;

#[derive(Debug)]
pub enum Node {
    FunctionDefinition { name: String, contents: Block, impure: bool }
}

pub type Block = Vec<Expr>;

#[derive(Debug, Clone)]
pub enum Expr {
    FunctionCall { name: String, arguments: Vec<Statement> }
}

#[derive(Debug, Clone)]
pub enum Statement {
    StringLiteral(String)
}

impl Parser {
    pub fn new(input: &str) -> Parser {
        Parser {
            lexer: Lexer::new(input)
        }
    }

    #[inline(always)]
    fn expect_token(&mut self, token: Token, consume: bool) -> Token {
        let next = if consume { self.lexer.next_token() } else { self.lexer.peek_token() };
        if let Some(n) = next {
            if discriminant(&n) == discriminant(&token) {
                return n;
            }
        }

        todo!("Handle unexpected token");
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.lexer.next_token() {
            Some(Token::StringLiteral(s)) => Some(Statement::StringLiteral(s)),
            Some(_) => todo!("Unexpected token in parse_statement"),
            None => None
        }
    }

    fn parse_expr(&mut self) -> Option<Expr> {
        // Only function calls for now
        match self.lexer.next_token() {
            Some(Token::Identifier(s)) => {
                match self.lexer.peek_token() {
                    Some(Token::OpenParen) => {
                        self.lexer.next_token();
                        let mut args = Vec::<Statement>::new();
                        while let Some(tk) = self.lexer.peek_token() {
                            match tk {
                                Token::CloseParen => break,
                                _ => {
                                    if let Some(s) = self.parse_statement() {
                                        args.push(s);
                                    } else {
                                        // Hit EOF or something else before close bracket!
                                        todo!("Handle unclosed function call");
                                    }

                                    match self.lexer.peek_token() {
                                        Some(Token::CloseParen) => break,
                                        Some(Token::Comma) => self.lexer.next_token(),
                                        _ => todo!("Unexpected token after func call arg")
                                    };
                                }
                            };
                        }

                        self.expect_token(Token::CloseParen, true);
                        self.expect_token(Token::Semicolon, true);

                        Some(Expr::FunctionCall {
                            name: s,
                            arguments: args
                        })
                    },
                    Some(_) => {
                        todo!("Unexpected token in parse_expr after ident");
                    },
                    None => {
                        todo!("Handle identifier by itself with nothing after it");
                    }
                }
            },
            Some(_) => {
                todo!("Unexpected token in parse_expr"); 
            },
            None => None    
        }
    }

    fn parse_node(&mut self) -> Option<Node> {
        // Only option is a function definition (for now)
        match self.lexer.peek_token() {
            Some(Token::Keyword(Keyword::Impure)) => {
                self.lexer.next_token();
                match self.parse_node() {
                    Some(Node::FunctionDefinition { name, contents, impure: _ }) => {
                        Some(Node::FunctionDefinition { name, contents, impure: true })
                    },
                    #[allow(unreachable_patterns)]
                    Some(_) => todo!("Unexpected token after impure"),
                    None => todo!("Impure by itself!")
                }
            },
            Some(Token::Keyword(Keyword::Func)) => {
                self.lexer.next_token();
                let name = self.expect_token(Token::Identifier(String::new()), true);
                self.expect_token(Token::OpenParen, true);
                self.expect_token(Token::CloseParen, true);
                self.expect_token(Token::OpenCurly, true);

                let mut block = Block::new();

                while let Some(t) = self.lexer.peek_token() {
                    match t {
                        Token::CloseCurly => break,
                        _ => {
                            if let Some(e) = self.parse_expr() {
                                block.push(e);
                            } else {
                                // Hit EOF or something else before close curly!
                                todo!("Handle unclosed function definition");
                            }
                        }
                    }
                }

                self.expect_token(Token::CloseCurly, true);

                if let Token::Identifier(n) = name {
                    Some(Node::FunctionDefinition { name: n, contents: block, impure: false })
                } else {
                    todo!("Unreachable!");
                }
            },
            Some(_) => {
                // TODO: Proper errors
                todo!("Unexpected token: parse_node");
            },
            None => None
        }
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program::new();

        while let Some(node) = self.parse_node() {
            program.push(node);
        }

        program
    } 

    // Use up all tokens and print them
    #[allow(dead_code)]
    pub fn token_drought(&mut self) {
        while let Some(token) = self.lexer.next_token() {
            println!("{:?}", token);
        }
    }
}
