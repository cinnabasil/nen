mod builtin;
mod codegen;
mod error;
mod ir;
mod lexer;
mod parser;

use std::{io::{ Read, Write }, fs::File};

use parser::Parser;
use ir::IR;
use codegen::ir_bytecode;

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

    let bytecode = ir_bytecode(intermediate);
    
    let mut file = File::create("out.nenc").expect("couldn't create");
    file.write(&bytecode).expect("couldn't write");
}
