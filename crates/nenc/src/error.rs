use std::fmt;

const RED: &str = "\u{001b}[91m";
const RESET: &str = "\u{001b}[0m";

pub struct ErrorLocation {
    pub line: usize,
    pub column: usize
}

pub enum SyntaxError {
    UnknownStartOfToken(ErrorLocation, char)
}

impl fmt::Display for SyntaxError {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SyntaxError::UnknownStartOfToken(loc, c) => {
                write!(f, "{RED}ERROR{RESET} {}:{}: Unexpected start of token: {RED}{}{RESET}", loc.line, loc.column, c)
            }
        }
   } 
}
