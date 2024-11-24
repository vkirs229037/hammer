use crate::parser::ast::AstNode;
use crate::parser::tokens::TokenType;
use crate::vm::vm::Value;
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
    pub fn new(tree: &'static AstNode, out_file_path: String) -> io::Result<Self>  {
        let path = path::Path::new(&out_file_path);
        let file = fs::File::open(path)?;
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

    pub fn compile(&mut self) {
        self.compile_expr()
    }

    fn compile_expr(&mut self) -> io::Result<()> {
        match self.current_subtree {
            AstNode::Binary(left, op, right) => {
                self.current_subtree = left;
                self.compile_expr();
                self.current_subtree = right;
                self.compile_expr();
                match op.ttype {
                    TokenType::OpPlus => self.out_file.write(0x02),
                }
            }
        }
    }

    fn compile_term(&mut self) {
        self.compile_factor()

        
    }
}