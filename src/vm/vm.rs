use crate::vm::errors::*;
use crate::vm::instruction::*;

pub type Value = f64;

#[macro_use]
mod vm_macros {
    macro_rules! exec_binop {
        ($vm:ident, $op:tt) => {
            let a = $vm.pop_stack()?;
            let b = $vm.pop_stack()?;
            $vm.stack.push((b $op a).into());
            $vm.pc += 1;
        };
    }
}


pub struct VM {
    stack: Vec<Value>,
    program: Vec<u8>,
    consts: Vec<Value>,
    pc: usize,
    running: bool,
}

impl VM {
    pub fn new(bytecode: Vec<u8>) -> Self {
        let mut bytecode_iter = bytecode.into_iter();
        let program: Vec<u8> = bytecode_iter.by_ref().take_while(|c| *c != 0xff).chain([0xff]).collect();
        let const_table: Vec<u8> = bytecode_iter.skip(1).collect();
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

    fn parse_const_table(&self, const_table: Vec<u8>) -> Vec<Value> {
        vec![]
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
                let index: u16 = self.next_2_bytes()?;
                let val: Value = self.get_const(index as usize)?;
                self.stack.push(val);
                self.pc += 3;
                
            },
            Instruction::ADD => {
                exec_binop!(self, +);
            },
            Instruction::SUB => {
                exec_binop!(self, -);
            },
            Instruction::MUL => {
                exec_binop!(self, *);
            },
            Instruction::DIV => {
                let a = self.pop_stack()?;
                let b = self.pop_stack()?;
                if a == 0f64 {
                    return Err(InterpretationError::ZeroDivisionError);
                }
                self.stack.push(b / a);
                self.pc += 1;
            },
            Instruction::NEG => {
                let a = self.pop_stack()?;
                self.stack.push(-a);
                self.pc += 1;
            }
            Instruction::EQ => {
                exec_binop!(self, ==);
            },
            Instruction::NEQ => {
                exec_binop!(self, !=);
            },
            Instruction::GR => {
                exec_binop!(self, >);
            },
            Instruction::LS => {
                exec_binop!(self, <);
            },
            Instruction::GE => {
                exec_binop!(self, >=);
            },
            Instruction::LE => {
                exec_binop!(self, <=);
            },
            Instruction::JMP => {
                let offset: u16 = self.next_2_bytes()?;
                self.pc += offset as usize;
            },
            Instruction::JF => {
                let offset: u16 = self.next_2_bytes()?;
                let a = self.pop_stack()?;
                if a == 0f64 {
                    self.pc += offset as usize;
                } else {
                    self.pc += 3;
                }
            },
            Instruction::JBACK => {
                let offset: u16 = self.next_2_bytes()?;
                self.pc -= offset as usize;
            }
            Instruction::DBG => {
                let a = self.pop_stack()?;
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
                    .ok_or_else(|| InterpretationError::UnexpectedEndError)
                    .copied()
    }

    fn next_2_bytes(self: &VM) -> Result<u16, InterpretationError> {
        let b1: u8 = self.get_byte(1)?;
        let b2: u8 = self.get_byte(2)?;
        Ok(u16::from_ne_bytes([b1, b2]))
    }

    fn get_const(self: &VM, index: usize) -> Result<Value, InterpretationError> {
        self.consts.get(index as usize)
                   .ok_or_else(|| InterpretationError::BadConstsIndexError)
                   .copied()
    }

    fn pop_stack(self: &mut VM) -> Result<Value, InterpretationError> {
        self.stack.pop()
                  .ok_or_else(|| InterpretationError::EmptyStackError)
    }
}