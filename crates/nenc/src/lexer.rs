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
pub enum Token {
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
    }

    #[inline(always)]
    fn skip_whitespace(&mut self) {
        while !self.at_end() {
            if self.input[self.index].is_whitespace() {
                self.index += 1;
            } else {
                break;
            }
        }
    }

    #[inline(always)]
    fn tokenize_single_char(&mut self, token: Token) -> Option<Token> {
        self.advance();
        return Some(token);
    }

    pub fn peek_token(&mut self) -> Option<Token> {
        let index = self.index;
        let token = self.next_token();
        // Reset position
        self.index = index;
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
                let mut value = String::new();
                while !self.at_end() && (self.input[self.index].is_ascii_alphanumeric() || self.input[self.index] == '_') {
                    value.push(self.input[self.index]);
                    self.index += 1;
                }
                return match value.as_str() {
                    "func" => Some(Token::Keyword(Keyword::Func)),
                    "impure" => Some(Token::Keyword(Keyword::Impure)),
                    _ => Some(Token::Identifier(value))
                };
            },
            '"' | '\'' => {
                let mut value = String::new();
                self.index += 1;
                while !self.at_end() {
                    let string_character = self.input[self.index];
                    if string_character == c {
                        self.index += 1;
                        break;
                    }
                    value.push(string_character);
                    self.index += 1;
                }

                return Some(Token::StringLiteral(value));
            },
            '(' => return self.tokenize_single_char(Token::OpenParen),
            ')' => return self.tokenize_single_char(Token::CloseParen),
            '{' => return self.tokenize_single_char(Token::OpenCurly),
            '}' => return self.tokenize_single_char(Token::CloseCurly),
            ',' => return self.tokenize_single_char(Token::Comma),
            ';' => return self.tokenize_single_char(Token::Semicolon), 
            c => {
                let error = SyntaxError::UnknownStartOfToken(ErrorLocation { line: self.line, column: self.column }, c);

                println!("{error}");

                std::process::exit(1);
            }
        };
    }
}
