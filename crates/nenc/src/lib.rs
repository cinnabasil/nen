mod lexer;
mod parser;

use std::io::Read;
use parser::Parser;

// TODO
pub struct CompilerOptions {}

pub fn compile(mut readable: impl Read, _options: CompilerOptions) {
    // TODO: Chunking 
    let mut src = String::new();
    readable.read_to_string(&mut src).unwrap();

    let _parser = Parser::new(&src);
}
