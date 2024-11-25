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
    out_file: fs::File,
    file_name: String,
    const_table: Vec<Value>
}

impl Compiler<'_> {
    pub fn new(tree: &'static AstNode, out_file_path: String) -> Result<Self, CompileError>  {
        let path = path::Path::new(&out_file_path);
        let file = fs::File::open(path).map_err(|e| CompileError::FileError(out_file_path.clone(), e))?;
        let mut compiler = Self { 
            tree: &AstNode::None,
            current_subtree: &AstNode::None,
            out_file: file,
            file_name: out_file_path,
            const_table: vec![]
        };
        compiler.tree = tree;
        compiler.current_subtree = &compiler.tree;
        Ok(compiler)
    }

    pub fn compile(&mut self) -> Result<(), CompileError> {
        self.compile_expr()
    }

    fn compile_expr(&mut self) -> Result<(), CompileError> {
        match self.current_subtree {
            AstNode::Binary(left, op, right) => {
                self.current_subtree = left;
                self.compile_expr();
                self.current_subtree = right;
                self.compile_expr();
                match op.ttype {
                    TokenType::OpPlus => self.write_out(&[0x02]),
                    TokenType::OpMinus => self.write_out(&[0x03]),
                    TokenType::OpStar => self.write_out(&[0x04]),
                    TokenType::OpSlash => self.write_out(&[0x05]),
                    _ => Err(CompileError::ExpectedOp(op.loc.clone()))
                }
            },
            AstNode::Unary(op, expr) => {
                self.current_subtree = expr;
                self.compile_expr();
                match op.ttype { 
                    TokenType::OpMinus => self.write_out(&[0x06]),
                    _ => Err(CompileError::ExpectedOp(op.loc.clone()))
                }
            },
            AstNode::Grouping(expr) => {
                self.current_subtree = expr;
                self.compile_expr()
            },
            AstNode::Literal(val) => {
                let index = self.const_table.len();
                if index > u16::MAX as usize {
                    return Err(CompileError::ConstTableOverflow)
                }
                self.const_table.push(*val);
                self.write_out(&[0x01])?;
                self.write_out(&u16::to_le_bytes(index as u16))
            },
            AstNode::None => panic!("неожиданное появление AstNode::None"),
        }
    }

    fn write_out(&mut self, bytes: &[u8]) -> Result<(), CompileError> {
        self.out_file.write(bytes).map(|_| ()).map_err(|e| CompileError::FileError(self.file_name.clone(), e))
    }
}