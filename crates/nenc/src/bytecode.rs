use crate::ir::{IR, Constant, NamespaceElement, Function, IRExpr};

struct BytecodeGenerator {
    function_indexes: Vec<String>
}

impl BytecodeGenerator {
    fn generate_expr_bytecode(&self, expr: &IRExpr) -> Vec<u8> {
        let mut bytecode = Vec::<u8>::new();
        
        match expr {
            IRExpr::FunctionCall(name, arguments) => {
                let idx = match self.function_indexes.iter().position(|n| n == name) {
                    Some(idx) => idx,
                    None => panic!("(hopefully) Unreachable")
                };
                
                // Evaluate arguments first
                for arg in arguments {
                    bytecode.extend(self.generate_expr_bytecode(arg));
                }
                // We need some good way to access these later
                
                // Function call is 0x02 and argument is size of function index
                // in this case, 4 (u32)
                bytecode.extend(&[0x02]);
                bytecode.extend(idx.to_be_bytes());
            },
            IRExpr::StringLiteral(idx) => {
                // For a string, we push it onto the stack **for now**
                
                // 0xE4 [constant index]
                bytecode.extend(&[0xE4]);
                bytecode.extend(idx.to_be_bytes());
            }
        }
        
        bytecode
    }
    
    fn generate_function_body_bytecode(&self, body: &Vec<IRExpr>) -> Vec<u8> {
        let mut bytecode = Vec::<u8>::new();
        
        for expr in body {
            bytecode.extend(self.generate_expr_bytecode(expr));
        }
        
        bytecode
    }
}

pub fn generate_bytecode(ir: IR) -> Vec<u8> {
    let mut generator = BytecodeGenerator {
        // This is a list of all defined functions, and the index of the name indicates
        // the index in the main body section
        function_indexes: Vec::<String>::new()
    };
    
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
    
    // The 4-byte index of the `main` function is output directly after
    // the main code length bytes (and this index is included in that length)
    let mut main_function_idx: u32 = 0;
    let mut current_idx: u32 = 0;

    for name in ir.namespace.keys() {
        generator.function_indexes.push(name.to_string());
    }
    
    for name in &generator.function_indexes {
        // We can safely unwrap since only namespace items were pushed into the indexes
        // Later we will need to worry since we can't blindly shove everything into the 
        // function indexes, as variables will become part of the namespace, however it is
        // axiomatically true that they will all be functions *for now*
        let item = ir.namespace.get(name).unwrap();
        match item {
            NamespaceElement::Variable => todo!("Variables are not implemented yet"),
            NamespaceElement::Function(function) => {
                match function {
                    Function::ToBeDefined { argument_count: _ } => panic!("Unreachable"),
                    // FOR NOW (TODO) we are putting built in functions
                    // in the code, until i figure out a good way to separate them
                    Function::BuiltIn { arguments: _, impure } => {
                        main_code_section.extend((name.len() as u32).to_be_bytes());
                        main_code_section.extend(name.as_bytes());
                        
                        let mut attributes: u8 = 0;
                        
                        if *impure {
                            attributes |= 0x01;
                        }
                        
                        main_code_section.extend(attributes.to_be_bytes());
                        main_code_section.extend((0 as u32).to_be_bytes());
                    },
                    Function::UserDefined { arguments: _, body, impure } => {
                        if name == "main" {
                            main_function_idx = current_idx;
                        }
                        
                        // Function layout
                        // [name len (4 bytes)] [name] [attributes] [body len (4 bytes)] [body]
                        // attributes (ORed)
                        //     0x01 - impure
                        
                        main_code_section.extend((name.len() as u32).to_be_bytes());
                        main_code_section.extend(name.as_bytes());
                        
                        let mut attributes: u8 = 0;
                        
                        if *impure {
                            attributes |= 0x01;
                        }
                        
                        main_code_section.extend(attributes.to_be_bytes());
                        
                        // TODO: Body
                        #[allow(unused_variables)]
                        let body_code = generator.generate_function_body_bytecode(&body);
                        
                        main_code_section.extend((body_code.len() as u32).to_be_bytes());
                        main_code_section.extend(body_code);
                        
                        current_idx += 1;
                    }
                }
            }
        }
    }
    
    program.extend((main_code_section.len() as u32).to_be_bytes());
    program.extend(main_function_idx.to_be_bytes());
    program.extend(main_code_section);
    
    // 4 bytes to indicate program length vvv
    bytecode.extend((program.len() as u32).to_be_bytes());
    bytecode.extend(&program);
    
    bytecode
}
