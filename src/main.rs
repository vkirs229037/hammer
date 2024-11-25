mod vm;
mod parser;
mod utils;
mod compile;

use parser::lexer::Lexer;
use parser::ast::{AstBuilder, AstNode};
use compile::compiler::Compiler;

fn main() {
    /*
    let mut vm = VM::new();
    vm.add_const(20f64);
    vm.add_const(10f64);
    // if (20 > 10) { 20 + 10 } else { 20 - 10 }
    let program: Vec<u8> = vec![
        0x01, 0x00, 0x00, // PUSH 0 -> 20
        0x01, 0x01, 0x00, // PUSH 1 -> 20 10
        0x09,             // GR     -> 20 > 10 -> 1
        0x0e, 0x0c, 0x00, // JF 12
        0x01, 0x00, 0x00, // PUSH 0
        0x01, 0x01, 0x00, // PUSH 1
        0x02,             // ADD
        0xfe,             // DBG
        0xff,             // HLT
        0x01, 0x00, 0x00, // PUSH 0
        0x01, 0x01, 0x00, // PUSH 1
        0x03,             // SUB
        0xfe,             // DBG
        0xff,             // HLT
    ];
    vm.load_program(program);
    
    let run_result = vm.run();
    match run_result {
        Ok(()) => println!("Все хорошо:)"),
        Err(e) => println!("Ошибка: {e}"),
    }
    */
    let program = String::from("-4 / 3 + 6 * (7 - 1);");
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
    let mut compiler = match Compiler::new(&tree, String::from("../../out/out")) {
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
}
