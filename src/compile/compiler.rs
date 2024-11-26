use crate::parser::ast::AstNode;
use crate::parser::tokens::TokenType;
use crate::vm::vm::Value;
use crate::compile::errors::*;
use std::fs;
use std::path;
use std::io::{self, Write};

pub struct Compiler<'c> {
    tree: &'c AstNode,
    current_subtree: &'c AstNode,
    file_name: String,
    const_table: Vec<Value>
}

impl<'c> Compiler<'c> {
    pub fn new(tree: &'c AstNode, out_file_path: String) -> Result<Self, CompileError>  {
        let mut compiler = Self { 
            tree: &AstNode::None,
            current_subtree: &AstNode::None,
            file_name: out_file_path,
            const_table: vec![]
        };
        compiler.tree = tree;
        compiler.current_subtree = &compiler.tree;
        Ok(compiler)
    }

    pub fn compile(&mut self) -> Result<(), CompileError> {
        let path = path::Path::new(&self.file_name);
        let mut file = fs::OpenOptions::new().write(true)
                                             .create(true)
                                             .truncate(true)
                                             .open(path)
                                             .map_err(|e| CompileError::FileError(self.file_name.clone(), e))?;
        self.compile_expr(&mut file)?;
        self.write_out(&[0xfe], &mut file)?;
        self.write_out(&[0xff], &mut file)?;
        for c in &self.const_table {
            // TODO: Типы
            self.write_out(&[0x00, 0x08], &mut file)?;
            let bytes = f64::to_le_bytes(*c);
            self.write_out(&bytes, &mut file)?;
        }
        Ok(())
    }

    fn compile_expr(&mut self, file: &mut fs::File) -> Result<(), CompileError> {
        match self.current_subtree {
            AstNode::Binary(left, op, right) => {
                self.current_subtree = left;
                self.compile_expr(file);
                self.current_subtree = right;
                self.compile_expr(file);
                match op.ttype {
                    TokenType::OpPlus => self.write_out(&[0x02], file),
                    TokenType::OpMinus => self.write_out(&[0x03], file),
                    TokenType::OpStar => self.write_out(&[0x04], file),
                    TokenType::OpSlash => self.write_out(&[0x05], file),
                    _ => Err(CompileError::ExpectedOp(op.loc.clone()))
                }
            },
            AstNode::Unary(op, expr) => {
                self.current_subtree = expr;
                self.compile_expr(file);
                match op.ttype { 
                    TokenType::OpMinus => self.write_out(&[0x06], file),
                    _ => Err(CompileError::ExpectedOp(op.loc.clone()))
                }
            },
            AstNode::Grouping(expr) => {
                self.current_subtree = expr;
                self.compile_expr(file)
            },
            AstNode::Literal(val) => {
                let index = self.const_table.len();
                if index > u16::MAX as usize {
                    return Err(CompileError::ConstTableOverflow)
                }
                self.const_table.push(*val);
                self.write_out(&[0x01], file)?;
                self.write_out(&u16::to_le_bytes(index as u16), file)
            },
            AstNode::None => panic!("неожиданное появление AstNode::None"),
        }
    }

    fn write_out(&self, bytes: &[u8], file: &mut fs::File) -> Result<(), CompileError> {
        file.write(bytes).map(|_| ()).map_err(|e| CompileError::FileError(self.file_name.clone(), e))
    }

    pub fn consts(&self) -> &Vec<f64> {
        &self.const_table
    }
}