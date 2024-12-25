use crate::vm::errors::*;

#[derive(Debug)]
pub enum Instruction {
    Nop,
    Push,
    Add,
    Sub,
    Mul,
    Div,
    Neg,
    Eq,
    Neq,
    Gr,
    Ls,
    Ge,
    Le,
    Jmp,
    Jf,
    Jback,
    Bin,
    Liv,
    Lfv,
    Dbg,
    Hlt,
}

impl TryFrom<u8> for Instruction {
    type Error = InterpretationError;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            0x00 => Ok(Instruction::Nop),
            0x01 => Ok(Instruction::Push),
            0x02 => Ok(Instruction::Add),
            0x03 => Ok(Instruction::Sub),
            0x04 => Ok(Instruction::Mul),
            0x05 => Ok(Instruction::Div),
            0x06 => Ok(Instruction::Neg),
            0x07 => Ok(Instruction::Jmp),
            0x08 => Ok(Instruction::Eq),
            0x09 => Ok(Instruction::Neq),
            0x0a => Ok(Instruction::Gr),
            0x0b => Ok(Instruction::Ls),
            0x0c => Ok(Instruction::Ge),
            0x0d => Ok(Instruction::Le),
            0x0e => Ok(Instruction::Jmp),
            0x0f => Ok(Instruction::Jf),
            0x10 => Ok(Instruction::Jback),
            0x11 => Ok(Instruction::Bin),
            0x12 => Ok(Instruction::Liv),
            0x13 => Ok(Instruction::Lfv),
            0xfe => Ok(Instruction::Dbg),
            0xff => Ok(Instruction::Hlt),
            _ => Err(InterpretationError::OpcodeError),
        }
    }
}
