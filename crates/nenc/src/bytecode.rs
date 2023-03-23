use std::collections::HashMap;

use crate::ir::{IR, Constant, NamespaceElement, Function};

pub fn generate_bytecode(ir: IR) -> Vec<u8> {
    let mut bytecode = Vec::<u8>::new();
    let mut program = Vec::<u8>::new();

    let magic_bytes: &[u8; 4] = &[0x4E, 0x45, 0x4E, 0x43];
    bytecode.extend(magic_bytes);

    // 0x00 0x00 -> Start of constant section
    program.extend(&[0x00, 0x00]);
    // Constants have a different format depending on type
    // String:
    // [type (0x01)] [string length (4 bytes)] [string]
    
    for constant in ir.constants {
        match constant {
            Constant::StringLiteral(s) => {
            program.extend(&[0x01]);
            program.extend([&(s.len() as u32).to_be_bytes(), s.as_bytes()].concat());
            }
        };
    }

    // 0x00 0x01 -> Start of main code section
    // followed by 4 bytes of main code length
    let mut main_code_section = Vec::<u8>::new(); 
    program.extend(&[0x00, 0x01]);

    // This hashmap contains a mapping of function name to index
    // in this section
    // The 4-byte index of the `main` function is output directly after
    // the main code length bytes (and this index is included in that
    // length)
    let mut bytecode_namespace = HashMap::<String, u32>::new();
    let mut main_function_idx: u32 = 0;
    let mut current_idx: u32 = 0;
    
    for (name, item) in ir.namespace {
        match item {
            NamespaceElement::Variable => todo!("Variables are not implemented yet"),
            NamespaceElement::Function(function) => {
                match function {
                    Function::ToBeDefined { argument_count: _ } => panic!("Unreachable"),
                    Function::BuiltIn { arguments: _, impure: _ } => {},
                    Function::UserDefined { arguments: _, body, impure } => {
                        if name == String::from("main") {
                            main_function_idx = current_idx;
                        }

                        bytecode_namespace.insert(name.clone(), current_idx);

                        // Function layout
                        // [name len (4 bytes)] [name] [attributes] [body len (4 bytes)] [body]
                        // attributes (ORed)
                        //     0x01 - impure

                        main_code_section.extend((name.len() as u32).to_be_bytes());
                        main_code_section.extend(name.as_bytes());

                        let mut attributes: u8 = 0;

                        if impure {
                            attributes |= 0x01;
                        }

                        main_code_section.extend(attributes.to_be_bytes());

                        // TODO: Body
                        main_code_section.extend((0 as u32).to_be_bytes());
                        
                        current_idx += 1;
                    }
                }
            }
        }
    }



    program.extend((main_code_section.len() as u32).to_be_bytes());
    program.extend(main_function_idx.to_be_bytes());
    
    // 4 bytes to indicate program length vvv
    bytecode.extend((program.len() as u32).to_be_bytes());
    bytecode.extend(&program);
    
    bytecode
}
