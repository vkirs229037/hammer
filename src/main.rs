mod vm;
mod parser;
mod utils;
mod compile;
mod cli;

use parser::lexer::Lexer;
use parser::ast::{AstBuilder, AstNode};
use compile::compiler::Compiler;
use vm::vm::VM;

use std::fs;
use std::io::Read;
use std::env;

fn main() {
    println!("a");
}

fn run_hammer_example() {
    let program = String::from("4 + 5.1 * 9 - 11.6;");

    let mut lexer = Lexer::new("module".to_owned(), program);
    match lexer.lex() {
        Ok(()) => {
            println!("Все хорошо:)");
            let tokens = lexer.tokens();
            for token in tokens {
                println!("{token}")
            }
        },
        Err(e) => println!("Ошибка: {e}")
    }

    let mut ast_builder = AstBuilder::new(lexer.tokens().to_vec());
    match ast_builder.parse() {
        Ok(()) => { 
            println!("Все хорошо:)");
            let tree = ast_builder.tree();
            println!("{tree:#?}");
        },
        Err(e) => println!("Ошибка: {e}"),
    };

    let tree = ast_builder.tree();
    let mut compiler = match Compiler::new(&tree, String::from("out/out")) {
        Ok(c) => c,
        Err(e) => { 
            println!("ошибка: {e}"); 
            return 
        }
    };
    match compiler.compile() {
        Ok(()) => println!("Все хорошо:)"),
        Err(e) => println!("Ошибка: {e}"),
    }

    let mut file = match fs::OpenOptions::new().read(true).open("out/out") {
        Ok(f) => f,
        Err(e) => panic!("{e}"),
    };
    let mut bytecode: Vec<u8> = vec![];
    file.read_to_end(&mut bytecode);
    let mut vm = match VM::new(bytecode) {
        Ok(v) => v,
        Err(e) => panic!("Ошибка: {e}"),
    };
    match vm.run() {
        Ok(()) => println!("Все хорошо:)"),
        Err(e) => println!("Ошибка: {e}"),
    }
}
