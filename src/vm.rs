use crate::instruction::*;
use crate::errors::*;
type Value = f64;

#[macro_use]
mod vm_macros {
    macro_rules! exec_jump {
        ($vm:ident, $op:tt) => {
            let offset: u16 = $vm.next_2_bytes()?;

            let a = $vm.pop_stack()?;
            let b = $vm.pop_stack()?;
                    
            if b $op a {
                $vm.pc += offset as usize;
            } else {
                $vm.pc += 3;
            }
        }
    }

    macro_rules! exec_binop {
        ($vm:ident, $op:tt) => {
            let a = $vm.pop_stack()?;
            let b = $vm.pop_stack()?;
            $vm.stack.push(a $op b);
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
                let index: u16 = self.next_2_bytes()?;
                let val: Value = self.get_const(index as usize)?;
                self.stack.push(val);
                self.pc += 3;
                
            },
            Instruction::ADD => {
                exec_binop!(self, +);
                self.pc += 1;
            },
            Instruction::SUB => {
                exec_binop!(self, -);
                self.pc += 1;
            },
            Instruction::MUL => {
                exec_binop!(self, *);
                self.pc += 1;
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
            Instruction::JMP => {
                let offset: u16 = self.next_2_bytes()?;
                self.pc += offset as usize;
            },
            Instruction::JE => {
                exec_jump!(self, ==);
            },
            Instruction::JNE => {
                exec_jump!(self, !=);
            },
            Instruction::JG => {
                exec_jump!(self, >);
            },
            Instruction::JL => {
                exec_jump!(self, <);
            },
            Instruction::JGE => {
                exec_jump!(self, >=);
            },
            Instruction::JLE => {
                exec_jump!(self, <=);
            },
            Instruction::RET => todo!("Не реализованы"),
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