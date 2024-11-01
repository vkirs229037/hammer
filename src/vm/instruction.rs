use crate::vm::errors::*;

#[derive(Debug)]
pub enum Instruction {
    NOP,
    PUSH,
    ADD,
    SUB,
    MUL,
    DIV,
    EQ,
    NEQ,
    GR,
    LS,
    GE,
    LE,
    JMP,
    JF,
    JBACK,
    DBG,
    HLT,
}

impl TryFrom<u8> for Instruction {
    type Error = InterpretationError;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            0x00 => Ok(Instruction::NOP),
            0x01 => Ok(Instruction::PUSH),
            0x02 => Ok(Instruction::ADD),
            0x03 => Ok(Instruction::SUB),
            0x04 => Ok(Instruction::MUL),
            0x05 => Ok(Instruction::DIV),
            0x06 => Ok(Instruction::JMP),
            0x07 => Ok(Instruction::EQ),
            0x08 => Ok(Instruction::NEQ),
            0x09 => Ok(Instruction::GR),
            0x0a => Ok(Instruction::LS),
            0x0b => Ok(Instruction::GE),
            0x0c => Ok(Instruction::LE),
            0x0d => Ok(Instruction::JMP),
            0x0e => Ok(Instruction::JF),
            0x0f => Ok(Instruction::JBACK),
            0xfe => Ok(Instruction::DBG),
            0xff => Ok(Instruction::HLT),
            _ => Err(InterpretationError::OpcodeError),
        }
    }
}