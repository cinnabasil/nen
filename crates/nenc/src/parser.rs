use std::mem::discriminant;

use lexer::{Lexer, Token, TokenKind, Keyword};

#[derive(Debug)]
pub struct Parser {
    lexer: Lexer
}


pub type Program = Vec<Node>;

#[derive(Debug)]
pub enum Node {
    FunctionDefinition { name: String, contents: Block, impure: bool }
}

pub type Block = Vec<Statement>;

#[derive(Debug, Clone)]
pub enum Expr {
    FunctionCall { name: String, arguments: Vec<Expr> },
    StringLiteral(String)
}

#[derive(Debug, Clone)]
pub enum Statement {
    Expr(Expr)
}

impl Parser {
    pub fn new(input: &str) -> Parser {
        Parser {
            lexer: Lexer::new(input)
        }
    }
    
    #[inline(always)]
    fn expect_token(&mut self, token: TokenKind, consume: bool) -> Token {
        let next = if consume { self.lexer.next_token() } else { self.lexer.peek_token() };
        if let Some(n) = next {
            if discriminant(&n.kind) == discriminant(&token) {
                return n;
            }
        }
        
        todo!("Handle unexpected token");
    }
    
    fn parse_statement(&mut self) -> Option<Statement> {
        match self.lexer.peek_token() {
            Some(token) => {
                match token.kind {
                    TokenKind::Identifier(_) => {
                        if let Some(t) = self.lexer.peek_second_token() {
                            match t.kind {
                                TokenKind::OpenParen => {
                                    let function_call = self.parse_expr().expect("Unreachable, I think");

                                    return Some(Statement::Expr(function_call));
                                },
                                _ => todo!()
                            }
                        } else {
                            todo!()
                        }
                    },
                    t => todo!("Unexpected token {t:?} in parse_statement"),
                }
            }
            None => None
        }
    }
    
    fn parse_expr(&mut self) -> Option<Expr> {
        // Only function calls for now
        match self.lexer.next_token() {
            Some(token) => {
                match token.kind {
                    TokenKind::StringLiteral(s) => {
                        Some(Expr::StringLiteral(s))
                    },
                    TokenKind::Identifier(s) => {
                        match self.lexer.peek_token() {
                            Some(peeked_token) => {
                                match peeked_token.kind {
                                    TokenKind::OpenParen => {
                                        self.lexer.next_token();
                                        let mut args = Vec::<Expr>::new();
                                        while let Some(tk) = self.lexer.peek_token() {
                                            match tk.kind {
                                                TokenKind::CloseParen => break,
                                                _ => {
                                                    if let Some(s) = self.parse_expr() {
                                                        args.push(s);
                                                    } else {
                                                        // Hit EOF or something else before close bracket!
                                                        todo!("Handle unclosed function call");
                                                    }
                                                    
                                                    match self.lexer.peek_token() {
                                                        Some(token) => {
                                                            match token.kind {
                                                                TokenKind::CloseParen => break,
                                                                TokenKind::Comma => self.lexer.next_token(),
                                                                _ => todo!("Unexpected token after func call arg")
                                                            }
                                                        },
                                                        None => todo!("EOF before close of call")
                                                    };
                                                }
                                            };
                                        }
                                        
                                        self.expect_token(TokenKind::CloseParen, true);
                                        
                                        Some(Expr::FunctionCall {
                                            name: s,
                                            arguments: args
                                        })
                                    },
                                    k => {
                                        todo!("Unexpected token {k:?} in parse_expr after ident");
                                    },
                                }
                            },
                            None => {
                                todo!("Handle identifier by itself with nothing after it");
                            }
                        }
                    },
                    k => {
                        todo!("Unexpected token {k:?}"); 
                    },
                }
            },
            None => None
        }
    }
    
    fn parse_node(&mut self) -> Option<Node> {
        // Only option is a function definition (for now)
        match self.lexer.peek_token() {
            Some(token) => {
                match token.kind {
                    TokenKind::Keyword(Keyword::Impure) => {
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
                    TokenKind::Keyword(Keyword::Func) => {
                        self.lexer.next_token();
                        let name = self.expect_token(TokenKind::Identifier(String::new()), true);
                        self.expect_token(TokenKind::OpenParen, true);
                        self.expect_token(TokenKind::CloseParen, true);
                        self.expect_token(TokenKind::OpenCurly, true);
                        
                        let mut block = Block::new();
                        
                        while let Some(t) = self.lexer.peek_token() {
                            match t.kind {
                                TokenKind::CloseCurly => break,
                                _ => {
                                    if let Some(s) = self.parse_statement() {
                                        block.push(s);
                                        self.expect_token(TokenKind::Semicolon, true);
                                    } else {
                                        // Hit EOF or something else before close curly!
                                        todo!("Handle unclosed function definition");
                                    }
                                }
                            }
                        }
                        
                        self.expect_token(TokenKind::CloseCurly, true);
                        
                        if let TokenKind::Identifier(n) = name.kind {
                            Some(Node::FunctionDefinition { name: n, contents: block, impure: false })
                        } else {
                            todo!("Unreachable!");
                        }
                    },
                    k => {
                        // TODO: Proper errors
                        todo!("Unexpected token {k:?} in parse_node");
                    },
                }
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
