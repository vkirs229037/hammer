mod vm;
mod parser;
mod utils;
mod compile;

use parser::lexer::Lexer;
use parser::ast::{AstBuilder, AstNode};
use compile::compiler::Compiler;
use vm::vm::VM;

use std::fs;
use std::io::Read;

fn main() {
    let program = String::from("(25 / 5 * 5 - (6 - 5)) + 5 + (25 * 5 - 5 * (5 - 3));");

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

    let mut vm = VM::new();
    let mut file = match fs::OpenOptions::new().read(true).open("out/out") {
        Ok(f) => f,
        Err(e) => panic!("{e}"),
    };
    let mut program: Vec<u8> = vec![];
    file.read_to_end(&mut program);
    vm.load_program(program);
    for c in compiler.consts() {
        vm.add_const(*c);
    }
    match vm.run() {
        Ok(()) => println!("Все хорошо:)"),
        Err(e) => println!("Ошибка: {e}"),
    }
}
