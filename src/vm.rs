use crate::instruction::*;
use crate::errors::*;
type Value = f64;

pub struct VM {
    stack: Vec<Value>,
    program: Vec<u8>,
    consts: Vec<Value>,
    pc: usize,
    running: bool,
}

impl VM {
    pub fn new() -> Self {
        VM {
            stack: vec![],
            program: vec![],
            consts: vec![],
            pc: 0,
            running: false,
        }
    }

    pub fn add_const(&mut self, value: Value) {
        self.consts.push(value);
    }

    pub fn load_program(&mut self, program: Vec<u8>) {
        self.program = program;
    }

    pub fn run(&mut self) -> Result<(), InterpretationError> {
        self.running = true;
        while self.running {
            self.run_one_instr()?;
        }
        Ok(())
    }

    fn run_one_instr(&mut self) -> Result<(), InterpretationError> {
        let inst: Instruction = self.get_byte(0)?
                                    .try_into()?;
        match inst {
            Instruction::NOP => { },
            Instruction::PUSH => {
                let b1: u8 = self.get_byte(1)?;
                let b2: u8 = self.get_byte(2)?;
                let index: u16 = u16::from_ne_bytes([b1, b2]);
                let val: Value = self.get_const(index as usize)?;
                self.stack.push(val);
                self.pc += 3;
                
            },
            Instruction::ADD => {
                let a = self.stack.pop()
                                  .ok_or_else(|| InterpretationError::EmptyStackError(EmptyStackError))?;
                let b = self.stack.pop()
                                  .ok_or_else(|| InterpretationError::EmptyStackError(EmptyStackError))?;
                self.stack.push(a + b);
                self.pc += 1;
            },
            Instruction::SUB => {
                let a = self.stack.pop()
                                  .ok_or_else(|| InterpretationError::EmptyStackError(EmptyStackError))?;
                let b = self.stack.pop()
                                  .ok_or_else(|| InterpretationError::EmptyStackError(EmptyStackError))?;
                self.stack.push(b - a);
                self.pc += 1;
            },
            Instruction::MUL => {
                let a = self.stack.pop()
                                  .ok_or_else(|| InterpretationError::EmptyStackError(EmptyStackError))?;
                let b = self.stack.pop()
                                  .ok_or_else(|| InterpretationError::EmptyStackError(EmptyStackError))?;
                self.stack.push(a * b);
                self.pc += 1;
            },
            Instruction::DIV => {
                let a = self.stack.pop()
                                  .ok_or_else(|| InterpretationError::EmptyStackError(EmptyStackError))?;
                let b = self.stack.pop()
                                  .ok_or_else(|| InterpretationError::EmptyStackError(EmptyStackError))?;
                if a == 0f64 {
                    return Err(InterpretationError::ZeroDivisionError(ZeroDivisionError));
                }
                self.stack.push(b / a);
                self.pc += 1;
            },
            Instruction::JMP => {
                let b1: u8 = self.get_byte(1)?;
                let b2: u8 = self.get_byte(2)?;
                let offset: u16 = u16::from_ne_bytes([b1, b2]);
                self.pc += offset as usize;
            },
            Instruction::JE => {
                let b1: u8 = self.get_byte(1)?;
                let b2: u8 = self.get_byte(2)?;
                let offset: u16 = u16::from_ne_bytes([b1, b2]);

                let a = self.stack.pop()
                                  .ok_or_else(|| InterpretationError::EmptyStackError(EmptyStackError))?;
                let b = self.stack.pop()
                                  .ok_or_else(|| InterpretationError::EmptyStackError(EmptyStackError))?;
                
                if b == a {
                    self.pc += offset as usize;
                } else {
                    self.pc += 3;
                }
            },
            Instruction::JNE => {
                let b1: u8 = self.get_byte(1)?;
                let b2: u8 = self.get_byte(2)?;
                let offset: u16 = u16::from_ne_bytes([b1, b2]);

                let a = self.stack.pop()
                                  .ok_or_else(|| InterpretationError::EmptyStackError(EmptyStackError))?;
                let b = self.stack.pop()
                                  .ok_or_else(|| InterpretationError::EmptyStackError(EmptyStackError))?;
                
                if b != a {
                    self.pc += offset as usize;
                } else {
                    self.pc += 3;
                }
            },
            Instruction::JG => {
                let b1: u8 = self.get_byte(1)?;
                let b2: u8 = self.get_byte(2)?;
                let offset: u16 = u16::from_ne_bytes([b1, b2]);

                let a = self.stack.pop()
                                  .ok_or_else(|| InterpretationError::EmptyStackError(EmptyStackError))?;
                let b = self.stack.pop()
                                  .ok_or_else(|| InterpretationError::EmptyStackError(EmptyStackError))?;
                
                if b > a {
                    self.pc += offset as usize;
                } else {
                    self.pc += 3;
                }
            },
            Instruction::JL => {
                let b1: u8 = self.get_byte(1)?;
                let b2: u8 = self.get_byte(2)?;
                let offset: u16 = u16::from_ne_bytes([b1, b2]);

                let a = self.stack.pop()
                                  .ok_or_else(|| InterpretationError::EmptyStackError(EmptyStackError))?;
                let b = self.stack.pop()
                                  .ok_or_else(|| InterpretationError::EmptyStackError(EmptyStackError))?;
                
                if b < a {
                    self.pc += offset as usize;
                } else {
                    self.pc += 3;
                }
            },
            Instruction::JGE => {
                let b1: u8 = self.get_byte(1)?;
                let b2: u8 = self.get_byte(2)?;
                let offset: u16 = u16::from_ne_bytes([b1, b2]);

                let a = self.stack.pop()
                                  .ok_or_else(|| InterpretationError::EmptyStackError(EmptyStackError))?;
                let b = self.stack.pop()
                                  .ok_or_else(|| InterpretationError::EmptyStackError(EmptyStackError))?;
                
                if b >= a {
                    self.pc += offset as usize;
                } else {
                    self.pc += 3;
                }
            },
            Instruction::JLE => {
                let b1: u8 = self.get_byte(1)?;
                let b2: u8 = self.get_byte(2)?;
                let offset: u16 = u16::from_ne_bytes([b1, b2]);

                let a = self.stack.pop()
                                  .ok_or_else(|| InterpretationError::EmptyStackError(EmptyStackError))?;
                let b = self.stack.pop()
                                  .ok_or_else(|| InterpretationError::EmptyStackError(EmptyStackError))?;
                
                if b <= a {
                    self.pc += offset as usize;
                } else {
                    self.pc += 3;
                }
            },
            Instruction::RET => todo!("Не реализованы"),
            Instruction::DBG => {
                let a = self.stack.pop()
                                  .ok_or_else(|| InterpretationError::EmptyStackError(EmptyStackError))?;
                println!("{a:#}");
                self.pc += 1;
            },
            Instruction::HLT => {
                self.running = false;
            },
        }
        Ok(())
    }

    fn get_byte(self: &VM, offset: usize) -> Result<u8, InterpretationError> {
        self.program.get(self.pc+offset)
                    .ok_or_else(|| InterpretationError::UnexpectedEndError(UnexpectedEndError))
                    .copied()
    }

    fn get_const(self: &VM, index: usize) -> Result<Value, InterpretationError> {
        self.consts.get(index as usize)
                   .ok_or_else(|| InterpretationError::BadConstsIndexError(BadConstsIndexError))
                   .copied()
    }
}