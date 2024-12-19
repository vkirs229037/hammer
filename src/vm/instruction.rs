use crate::vm::errors::*;

#[derive(Debug)]
pub enum Instruction {
    NOP,
    PUSH,
    ADD,
    SUB,
    MUL,
    DIV,
    NEG,
    EQ,
    NEQ,
    GR,
    LS,
    GE,
    LE,
    JMP,
    JF,
    JBACK,
    BIN,
    LIV,
    LFV,
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
            0x06 => Ok(Instruction::NEG),
            0x07 => Ok(Instruction::JMP),
            0x08 => Ok(Instruction::EQ),
            0x09 => Ok(Instruction::NEQ),
            0x0a => Ok(Instruction::GR),
            0x0b => Ok(Instruction::LS),
            0x0c => Ok(Instruction::GE),
            0x0d => Ok(Instruction::LE),
            0x0e => Ok(Instruction::JMP),
            0x0f => Ok(Instruction::JF),
            0x10 => Ok(Instruction::JBACK),
            0x11 => Ok(Instruction::BIN),
            0x12 => Ok(Instruction::LIV),
            0x13 => Ok(Instruction::LFV),
            0xfe => Ok(Instruction::DBG),
            0xff => Ok(Instruction::HLT),
            _ => Err(InterpretationError::OpcodeError),
        }
    }
}