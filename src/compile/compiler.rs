use crate::parser::ast::{Expr, Stmt, Variable};
use crate::parser::tokens::{TokenType, Token, BIn};
use crate::vm::vm::Value;
use crate::compile::errors::*;
use std::collections::HashMap;
use std::fs;
use std::path;
use std::io::{self, Write};
use std::rc::Rc;

pub struct Compiler {
    current_subtree: Option<Box<Expr>>,
    file_name: String,
    const_table: Vec<Value>,
    variable_numbers: HashMap<Variable, u32>,
    last_variable_number: u32,
}

impl Compiler {
    pub fn new(out_file_path: String) -> Result<Self, CompileError>  {
        let mut compiler = Self { 
            current_subtree: None,
            file_name: out_file_path,
            const_table: vec![],
            variable_numbers: HashMap::new(),
            last_variable_number: 0
        };
        Ok(compiler)
    }

    pub fn compile(&mut self, tree: Vec<Stmt>, variables: Vec<Variable>) -> Result<(), CompileError> {
        let path = path::Path::new(&self.file_name);
        let mut file = fs::OpenOptions::new().write(true)
                                             .create(true)
                                             .truncate(true)
                                             .open(path)
                                             .map_err(|e| CompileError::FileError(self.file_name.clone(), e))?;
        for stmt in tree {
            match stmt {
                Stmt::Expr(e) => {
                    self.current_subtree = Some(e);
                    self.compile_expr(&mut file, &variables)?;
                },
                Stmt::Block(_) => todo!("Блоки выражений"),
                Stmt::Decl(var, expr) => self.compile_decl(&mut file, var, expr, &variables)?,
                Stmt::Reassign(var, expr) => self.compile_reassign(&mut file, var, expr, &variables)?
            };
        }
        self.write_out(&[0xff], &mut file)?;
        for c in &self.const_table {
            // TODO: Типы
            self.write_out(&[0x00, 0x08], &mut file)?;
            let bytes = f64::to_le_bytes(*c);
            self.write_out(&bytes, &mut file)?;
        }
        Ok(())
    }

    fn compile_decl(&mut self, file: &mut fs::File, var: Variable, expr: Box<Expr>, variables: &Vec<Variable>) -> Result<(), CompileError> {
        self.current_subtree = Some(expr);
        self.compile_expr(file, variables)?;
        self.variable_numbers.insert(var, self.last_variable_number);
        self.write_out(&[0x12], file)?;
        let idx: [u8; 4] = u32::to_le_bytes(self.last_variable_number);
        self.last_variable_number += 1;
        self.write_out(&idx, file)?;
        Ok(())
    }

    fn compile_reassign(&mut self, file: &mut fs::File, var: Variable, expr: Box<Expr>, variables: &Vec<Variable>) -> Result<(), CompileError> {
        self.current_subtree = Some(expr);
        self.compile_expr(file, variables)?;
        let var_number = self.variable_numbers[&var];
        self.write_out(&[0x12], file)?;
        let idx: [u8; 4] = u32::to_le_bytes(var_number);
        self.write_out(&idx, file)?;
        Ok(())
    }

    fn compile_expr(&mut self, file: &mut fs::File, variables: &Vec<Variable>) -> Result<(), CompileError> {
        // В идеале здесь не должно быть клонирования, однако я просто
        // уже не знаю как по другому сделать((
        match *self.current_subtree.clone().unwrap() {
            Expr::Binary(left, op, right) => {
                self.current_subtree = Some(left);
                self.compile_expr(file, variables);
                self.current_subtree = Some(right);
                self.compile_expr(file, variables);
                match op.ttype {
                    TokenType::OpPlus => self.write_out(&[0x02], file),
                    TokenType::OpMinus => self.write_out(&[0x03], file),
                    TokenType::OpStar => self.write_out(&[0x04], file),
                    TokenType::OpSlash => self.write_out(&[0x05], file),
                    _ => Err(CompileError::ExpectedOp(op.loc.clone()))
                }
            },
            Expr::Unary(op, expr) => {
                self.current_subtree = Some(expr);
                self.compile_expr(file, variables);
                match op.ttype { 
                    TokenType::OpMinus => self.write_out(&[0x06], file),
                    _ => Err(CompileError::ExpectedOp(op.loc.clone()))
                }
            },
            Expr::Grouping(expr) => {
                self.current_subtree = Some(expr);
                self.compile_expr(file, variables)
            },
            Expr::Literal(val) => {
                let index = self.const_table.len();
                if index > u16::MAX as usize {
                    return Err(CompileError::ConstTableOverflow)
                }
                let Token { ttype: TokenType::NumLit(value), loc: _ } = val else {
                    panic!("Невозможная ситуация: refutable pattern")
                };
                self.const_table.push(value);
                self.write_out(&[0x01], file)?;
                self.write_out(&u16::to_le_bytes(index as u16), file)
            },
            Expr::Func(func, expr) => {
                self.current_subtree = Some(expr);
                self.compile_expr(file, variables);
                match func.ttype {
                    TokenType::Builtin(bin) => {
                        self.write_out(&[0x11], file)?;
                        match bin {
                            BIn::Println => self.write_out(&[0x00, 0x00], file),
                            BIn::Abs => self.write_out(&[0x01, 0x00], file),
                        }
                    },
                    _ => todo!("Неопределенная функция {func:?}"),
                }
            },
            Expr::Variable(var, loc) => {
                let idx = *self.variable_numbers.get(&var).expect("На этапе построения дерева должно было быть определено, что эта переменная не объявлена");
                self.write_out(&[0x13], file);
                let idx_bytes = u32::to_le_bytes(idx); 
                self.write_out(&idx_bytes, file)
            }
            Expr::None => panic!("неожиданное появление AstNode::None"),
        }
    }

    fn write_out(&self, bytes: &[u8], file: &mut fs::File) -> Result<(), CompileError> {
        file.write(bytes).map(|_| ()).map_err(|e| CompileError::FileError(self.file_name.clone(), e))
    }

    pub fn consts(&self) -> &Vec<f64> {
        &self.const_table
    }
}