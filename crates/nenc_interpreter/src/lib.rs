use std::{io::Read, collections::HashMap};

#[derive(Debug)]
struct Interpreter {
    stack: Vec<StackElement>,
    scope: HashMap<String, Vec<Instruction>>
}

#[derive(Debug, Clone)]
enum StackElement {
    String(String)
}

#[derive(Debug, Clone)]
enum Instruction {
    PushString(String),
    Write,
    Call(String)
}

fn instruction_from_opcode(opcode: u8) -> Option<Instruction> {
   match opcode {
        0x12 => Some(Instruction::Write),
        0xA1 => Some(Instruction::Call(String::new())),
        0xE1 => Some(Instruction::PushString(String::new())),
        _ => None
    } 
}

fn get_byte_string_from_idx(code: &[u8], idx: &mut usize) -> String {
    let string_len_bytes = &code[*idx..*idx+2];
    let string_len = u16::from_be_bytes([
        string_len_bytes[0],
        string_len_bytes[1]
    ]) as usize;

    if *idx + 2 + string_len > code.len() {
        panic!("String out of bounds!");
    } 

    *idx += 2;

    let string_bytes = &code[*idx..*idx+string_len];
    let string = std::str::from_utf8(string_bytes).unwrap();

    *idx += string_len;

    string.to_string()
}

fn parse_instructions(instructions: &[u8]) -> Vec<Instruction> {
    let mut instructions_vec = Vec::<Instruction>::new();

    let mut idx = 0;

    while idx < instructions.len() {
        idx += 1;
        match instruction_from_opcode(instructions[idx - 1]) {
            Some(inst) => {
                match inst {
                    Instruction::Write => instructions_vec.push(Instruction::Write),
                    Instruction::Call(_) => {
                        let str = get_byte_string_from_idx(instructions, &mut idx);
                        instructions_vec.push(Instruction::Call(str));
                    },
                    Instruction::PushString(_) => {
                        let str = get_byte_string_from_idx(instructions, &mut idx);
                        instructions_vec.push(Instruction::PushString(str));
                    }
                }
            },
            None => panic!("Unrecognized opcode: {:?}", instructions[idx - 1])
        }
    }

    instructions_vec
}

impl Interpreter {
    fn new() -> Self {
        Interpreter {
            stack: Vec::<StackElement>::new(),
            scope: HashMap::<String, Vec<Instruction>>::new()
        }
    }

    fn interpret_code(&mut self, bytes: Vec<u8>) {
       let mut idx = 0;

        if bytes.len() < 4 {
            panic!("Invalid NENC file");
        }

        let magic_bytes = &bytes[idx..idx+4];
        assert_eq!(magic_bytes, &[0x4E, 0x45, 0x4E, 0x43]);
        idx += 4;

        if idx + 4 > bytes.len() {
            panic!("Body length not found");
        }

        let body_len_bytes = &bytes[idx..idx+4];
        let _body_len = u32::from_be_bytes([
            body_len_bytes[0],
            body_len_bytes[1],
            body_len_bytes[2],
            body_len_bytes[3],
        ]);
        idx += 4;

       
        while idx < bytes.len() {
            let function_name_length_bytes = &bytes[idx..idx+2];
            let function_name_length = u16::from_be_bytes(
                [function_name_length_bytes[0], function_name_length_bytes[1]]
            ) as usize;

            if idx + 2 + function_name_length > bytes.len() {
                panic!("Length of function was longer than function body");
            }

            idx += 2;
            let function_name_bytes = &bytes[idx..idx+function_name_length];
            let function_name = std::str::from_utf8(function_name_bytes).unwrap();
            idx += function_name_length;

            let function_body_length_bytes = &bytes[idx..idx+4];
            let function_body_length = u32::from_be_bytes([
                function_body_length_bytes[0],
                function_body_length_bytes[1],
                function_body_length_bytes[2],
                function_body_length_bytes[3],
            ]) as usize;

            if idx + 4 + function_body_length > bytes.len() {
                panic!("Length of function was longer than function body");
            }

            idx += 4;

            let function_body = parse_instructions(&bytes[idx..idx+function_body_length]);
            
            idx += function_body_length;

            self.scope.insert(function_name.to_string(), function_body);
      }
    }

    fn run_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Call(s) => self.run(s),
            Instruction::PushString(s) => self.stack.push(StackElement::String(s.to_string())),
            Instruction::Write => {
                match self.stack.pop() {
                    Some(StackElement::String(s)) => println!("{s}"),
                    None => panic!("Stack underflow @ write instruction")
                }
            }
        } 
    }

    fn run(&mut self, function: &str) {
        let instructions = self.scope.get(function).expect("Couldn't find function!"); 

        for instruction in instructions.clone().iter() {
            self.run_instruction(instruction);
        }
    }
}

pub fn interpret(mut readable: impl Read) {
    let mut nenc: Vec<u8> = Vec::<u8>::new();
    readable.read_to_end(&mut nenc).expect("Couldn't read file");

    let mut interpreter = Interpreter::new();

    interpreter.interpret_code(nenc);

    if interpreter.scope.get("main").is_none() {
        panic!("No main function found!");
    }

    interpreter.run("main");
}
