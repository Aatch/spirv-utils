pub mod desc;
pub mod read;
pub mod instruction;
pub mod module;
pub mod parser;

#[derive(Clone, Debug)]
pub struct RawInstruction {
    pub opcode: u16,
    pub params: Vec<u32>
}
