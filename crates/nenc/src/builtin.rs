use codegen::instruction_opcode;
use ir::Instruction;

pub fn get_bytecode_of_function(name: String) -> Option<&'static [u8]> {
    match name.as_str() {
        "print" => Some(
            instruction_opcode(Instruction::Write)
        ),
        _ => None
    }
}
