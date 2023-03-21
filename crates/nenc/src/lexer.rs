#[derive(Debug)]
pub struct Lexer {
    input: Vec<char>,
    index: usize
}

#[derive(Debug)]
pub enum Token {
    Identifier(String),
    StringLiteral(String),
    OpenParen,
    CloseParen,
    Semicolon
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input: input.chars().collect::<Vec<char>>(),
            index: 0
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

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        if self.at_end() {
            return None;
        }

        let c = self.input[self.index];
        
        match c {
            c if c.is_ascii_alphabetic() => {
                let mut value = String::new();
                while !self.at_end() && self.input[self.index].is_ascii_alphanumeric() {
                    value.push(self.input[self.index]);
                    self.index += 1;
                }
                return Some(Token::Identifier(value))
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
            '(' => {
                let token = Token::OpenParen;
                self.advance();
                return Some(token);
            }, 
            ')' => {
                let token = Token::CloseParen;
                self.advance();
                return Some(token);
            }, 
            ';' => {
                let token = Token::Semicolon;
                self.advance();
                return Some(token);
            }, 
            _ => {
                // TODO: Proper error handling
                eprintln!("\u{0001}[91m;ERROR\u{0001}[0m; Unexpected character at position {}: '{}'",
                    self.index, c); 
                std::process::exit(1);
            }
        };
    }
}
