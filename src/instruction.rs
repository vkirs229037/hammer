use crate::errors::*;

pub enum Instruction {
    NOP,
    PUSH,
    ADD,
    SUB,
    MUL,
    DIV,
    JMP,
    JE,
    JNE,
    JG,
    JL,
    JGE,
    JLE,
    RET,
    DBG,
    HLT,
}

impl TryFrom<u8> for Instruction {
    type Error = OpcodeError;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            0x00 => Ok(Instruction::NOP),
            0x01 => Ok(Instruction::PUSH),
            0x02 => Ok(Instruction::ADD),
            0x03 => Ok(Instruction::SUB),
            0x04 => Ok(Instruction::MUL),
            0x05 => Ok(Instruction::DIV),
            0x06 => Ok(Instruction::JMP),
            0x07 => Ok(Instruction::JE),
            0x08 => Ok(Instruction::JNE),
            0x09 => Ok(Instruction::JG),
            0x0a => Ok(Instruction::JL),
            0x0b => Ok(Instruction::JGE),
            0x0c => Ok(Instruction::JLE),
            0x0d => Ok(Instruction::RET),
            0x0e => Ok(Instruction::DBG),
            0x0f => Ok(Instruction::HLT),
            _ => Err(OpcodeError),
        }
    }
}