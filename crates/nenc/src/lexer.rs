use crate::error::{ErrorLocation, SyntaxError};

#[derive(Debug)]
pub struct Lexer {
    input: Vec<char>,
    index: usize,
    line: usize,
    column: usize
}

#[derive(Debug)]
pub enum Keyword {
    Func,
    Impure
}

#[derive(Debug)]
pub enum TokenKind {
    Identifier(String),
    Keyword(Keyword),
    StringLiteral(String),
    OpenParen,
    CloseParen,
    OpenCurly,
    CloseCurly,
    Comma,
    Semicolon
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Token {
    line: usize,
    column: usize,
    pub kind: TokenKind
}

impl Token {
    pub fn new(line: usize, column: usize, kind: TokenKind) -> Self {
        Token {
            line,
            column,
            kind
        }
    }
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input: input.chars().collect::<Vec<char>>(),
            index: 0,
            line: 0,
            column: 0
        }
    }
    
    #[inline(always)]
    fn at_end(&self) -> bool {
        self.index >= self.input.len()
    }
    
    fn advance(&mut self) {
        self.index += 1;
        self.column += 1;
    }
    
    #[inline(always)]
    fn skip_whitespace(&mut self) {
        while !self.at_end() {
            if self.input[self.index].is_whitespace() {
                match self.input[self.index] {
                    '\n' => {
                        self.line += 1;
                        self.column = 0;
                    }, 
                    _ => {
                        self.column += 1
                    }
                };
                self.index += 1;
            } else {
                break;
            }
        }
    }
    
    #[inline(always)]
    fn tokenize_single_char(&mut self, kind: TokenKind) -> Option<Token> {
        let token = Token::new(self.line, self.column, kind);
        self.advance();
        return Some(token);
    }
    
    pub fn peek_token(&mut self) -> Option<Token> {
        let index = self.index;
        let line = self.line;
        let column = self.column;
        let token = self.next_token();
        // Reset position
        self.index = index;
        self.line = line;
        self.column = column;
        return token;
    }

    pub fn peek_second_token(&mut self) -> Option<Token> {
        let index = self.index;
        let line = self.line;
        let column = self.column;
        self.next_token();
        let token = self.next_token();
        self.index = index;
        self.line = line;
        self.column = column;
        return token;
    }
    
    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        
        if self.at_end() {
            return None;
        }
        
        let c = self.input[self.index];
        
        match c {
            c if c.is_ascii_alphabetic() || c == '_' => {
                let line = self.line;
                let column = self.column;

                let mut value = String::new();
                while !self.at_end() && (self.input[self.index].is_ascii_alphanumeric() || self.input[self.index] == '_') {
                    value.push(self.input[self.index]);
                    self.advance();
                }
                return match value.as_str() {
                    "func" => Some(Token::new(line, column, TokenKind::Keyword(Keyword::Func))),
                    "impure" => Some(Token::new(line, column, TokenKind::Keyword(Keyword::Impure))),
                    _ => Some(Token::new(line, column, TokenKind::Identifier(value)))
                };
            },
            '"' | '\'' => {
                let line = self.line;
                let column = self.column;

                let mut value = String::new();
                self.advance();
                while !self.at_end() {
                    let string_character = self.input[self.index];
                    if string_character == c {
                        self.advance();
                        break;
                    }
                    value.push(string_character);
                    self.advance();
                }
                
                return Some(Token::new(line, column, TokenKind::StringLiteral(value)));
            },
            '(' => return self.tokenize_single_char(TokenKind::OpenParen),
            ')' => return self.tokenize_single_char(TokenKind::CloseParen),
            '{' => return self.tokenize_single_char(TokenKind::OpenCurly),
            '}' => return self.tokenize_single_char(TokenKind::CloseCurly),
            ',' => return self.tokenize_single_char(TokenKind::Comma),
            ';' => return self.tokenize_single_char(TokenKind::Semicolon), 
            c => {
                let error = SyntaxError::UnknownStartOfToken(ErrorLocation { line: self.line, column: self.column }, c);
                
                println!("{error}");
                
                std::process::exit(1);
            }
        };
    }
}
