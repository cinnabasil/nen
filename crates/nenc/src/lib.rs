mod bytecode;
mod error;
mod ir;
mod lexer;
mod parser;

use std::fs::File;
use std::io::{ Read, Write };

use bytecode::generate_bytecode;
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

//  println!("Program {program:#?}");

    let intermediate = IR::from(program);

    let bytecode = generate_bytecode(intermediate);

    let mut file = File::create("out.nenc").unwrap();
    file.write_all(&bytecode).unwrap();
}
