mod errors;
use errors::*;

type Value = f64;

enum Instruction {
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

struct VM {
    stack: Vec<Value>,
    program: Vec<u8>,
    consts: Vec<Value>,
    pc: usize,
    running: bool,
}

impl VM {
    fn new() -> Self {
        VM {
            stack: vec![],
            program: vec![],
            consts: vec![],
            pc: 0,
            running: false,
        }
    }

    fn add_const(&mut self, value: Value) {
        self.consts.push(value);
    }

    fn run(&mut self) -> Result<(), InterpretationError> {
        self.running = true;
        while self.running {
            self.run_one_instr()?;
        }
        Ok(())
    }

    fn run_one_instr(&mut self) -> Result<(), InterpretationError> {
        let inst: Instruction = self.program.get(self.pc)
                                            .ok_or_else(|| InterpretationError::UnexpectedEndError(UnexpectedEndError))?
                                            .clone()
                                            .try_into()?;
        match inst {
            Instruction::NOP => { },
            Instruction::PUSH => {
                let b1: u8 = *self.program.get(self.pc+1)
                                          .ok_or_else(|| InterpretationError::UnexpectedEndError(UnexpectedEndError))?;
                let b2: u8 = *self.program.get(self.pc+2)
                                          .ok_or_else(|| InterpretationError::UnexpectedEndError(UnexpectedEndError))?;
                let index: u16 = u16::from_ne_bytes([b1, b2]);
                let val: Value = *self.consts.get(index as usize)
                                             .ok_or_else(|| InterpretationError::BadConstsIndexError(BadConstsIndexError))?;
                self.stack.push(val);
                
            },
            Instruction::ADD => {
                let a = self.stack.pop()
                                  .ok_or_else(|| InterpretationError::EmptyStackError(EmptyStackError))?;
                let b = self.stack.pop()
                                  .ok_or_else(|| InterpretationError::EmptyStackError(EmptyStackError))?;
                self.stack.push(a + b);
            },
            Instruction::SUB => {
                let a = self.stack.pop()
                                  .ok_or_else(|| InterpretationError::EmptyStackError(EmptyStackError))?;
                let b = self.stack.pop()
                                  .ok_or_else(|| InterpretationError::EmptyStackError(EmptyStackError))?;
                self.stack.push(a - b);
            },
            Instruction::MUL => {
                let a = self.stack.pop()
                                  .ok_or_else(|| InterpretationError::EmptyStackError(EmptyStackError))?;
                let b = self.stack.pop()
                                  .ok_or_else(|| InterpretationError::EmptyStackError(EmptyStackError))?;
                self.stack.push(a * b);
            },
            Instruction::DIV => {
                let a = self.stack.pop()
                                  .ok_or_else(|| InterpretationError::EmptyStackError(EmptyStackError))?;
                let b = self.stack.pop()
                                  .ok_or_else(|| InterpretationError::EmptyStackError(EmptyStackError))?;
                self.stack.push(a / b);
            },
            Instruction::JMP => todo!("Не реализованы"),
            Instruction::JE => todo!("Не реализованы"),
            Instruction::JNE => todo!("Не реализованы"),
            Instruction::JG => todo!("Не реализованы"),
            Instruction::JL => todo!("Не реализованы"),
            Instruction::JGE => todo!("Не реализованы"),
            Instruction::JLE => todo!("Не реализованы"),
            Instruction::RET => todo!("Не реализованы"),
            Instruction::DBG => {
                let a = self.stack.pop()
                                  .ok_or_else(|| InterpretationError::EmptyStackError(EmptyStackError))?;
                println!("{a:#}");
            },
            Instruction::HLT => {
                self.running = false;
            },
        }
        Ok(())
    }
}

fn main() {
    println!("Hello, world!");
}
