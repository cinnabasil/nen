use ir::{ IR, Instruction, ScopeElement };
use builtin::get_bytecode_of_function;

pub fn instruction_opcode(instruction: Instruction) -> &'static [u8] {
    match instruction {
        Instruction::Write => &[0x12],
        Instruction::Call(_) => &[0xA1],
        Instruction::PushString(_) => &[0xE1],
    }
}

pub fn instruction_operand(instruction: Instruction) -> Vec<u8> {
    match instruction {
        Instruction::PushString(string) |
        Instruction::Call(string) => {
            [
                (string.len() as u16).to_be_bytes().to_vec(),
                string.as_bytes().to_vec()
            ].concat()
        },
        _ => Vec::<u8>::new() 
    }
}

pub fn ir_bytecode(mut ir: IR) -> Vec<u8> {
    let header: &[u8; 4] = &[0x4E, 0x45, 0x4E, 0x43];

    let mut bytecode = Vec::<u8>::new();

    if let Some(scope) = ir.scope.pop() {
        if scope.get("main").is_none() {
            panic!("No main function defined!");
        }

        for (name, element) in scope {
            match element {
                ScopeElement::Variable => todo!("Handle variables"),
                ScopeElement::PlaceholderFunction => panic!("Unreachable"),
                ScopeElement::Function(f) => {
                    let name_len: &[u8] = &(name.len() as u16).to_be_bytes();

                    let mut body = Vec::<u8>::new();

                    for instruction in f {
                        let opcode = instruction_opcode(instruction.clone());
                        body.extend(opcode);

                        let operand = instruction_operand(instruction.clone());
                        body.extend(operand);
                    }

                    let body_len: &[u8] = &(body.len() as u32).to_be_bytes();

                    let function_bytecode = [
                        name_len, 
                        name.as_bytes(), 
                        body_len, 
                        &body
                    ].concat();

                    bytecode.extend(function_bytecode);
                },
                ScopeElement::BuiltInFunction => {
                    match get_bytecode_of_function(name.clone()) {
                        Some(b) => {
                            let name_len: &[u8] = &(name.len() as u16).to_be_bytes();
                            let body_len: &[u8] = &(b.len() as u32).to_be_bytes();

                            let function_bytecode = [
                                name_len, 
                                name.as_bytes(), 
                                body_len, 
                                b
                            ].concat();

                            bytecode.extend(function_bytecode);
                        },
                        None => todo!("Built-in {name} called without codegen definition")
                    }
                }
            }
        }
    } else {
        panic!("Unreachable, since we should always finish IR generation with 1 scope exactly")
    }

    let mut full_bytecode: Vec<u8> = header.to_vec();
    full_bytecode.extend((bytecode.len() as u32).to_be_bytes());
    full_bytecode.extend(bytecode);

    full_bytecode
}
