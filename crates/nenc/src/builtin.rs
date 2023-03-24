use codegen::instruction_opcode;
use ir::Instruction;

pub fn get_bytecode_of_function(name: String) -> Option<Vec<u8>> {
    match name.as_str() {
        "print" => Some(
            instruction_opcode(Instruction::Write).to_vec()
        ),
        "println" => Some(
            [
                instruction_opcode(Instruction::Write),
                instruction_opcode(Instruction::PushString(String::new())),
                &[0x00, 0x01, 0x0A], // New Line length, and New Line
                instruction_opcode(Instruction::Write)
            ].concat()
        ),
        _ => None
    }
}
