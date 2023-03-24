mod error;
mod ir;
mod lexer;
mod parser;

use std::io::Read;

use parser::Parser;
use ir::IR;

// TODO
pub struct CompilerOptions {}

pub fn compile(mut readable: impl Read, _options: CompilerOptions) {
    // TODO: Chunking 
    let mut src = String::new();
    readable.read_to_string(&mut src).unwrap();
    
    let mut parser = Parser::new(&src);
    let program = parser.parse_program();
    
//    println!("Program {program:#?}");
    
    let intermediate = IR::from(program);
    println!("{intermediate:#?}");
}
