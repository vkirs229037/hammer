mod errors;
mod instruction;
mod vm;

use vm::*;

fn main() {
    let mut vm = VM::new();
    vm.add_const(20f64);
    vm.add_const(10f64);
    // if (20 > 10) { 20 + 10 } else { 20 - 10 }
    let _program1: Vec<u8> = vec![
        0x01, 0x00, 0x00, // PUSH 0 -> 20
        0x01, 0x01, 0x00, // PUSH 1 -> 20 10
        0x09, 0x0c, 0x00, // JG 12 
        0x01, 0x00, 0x00, // PUSH 0 -> 20
        0x01, 0x01, 0x00, // PUSH 1 -> 20 10
        0x03,             // SUB -> 10
        0x0e,             // DBG -> 10
        0x0f,             // HLT
        0x01, 0x00, 0x00, // PUSH 0 -> 20
        0x01, 0x01, 0x00, // PUSH 1 -> 20 10
        0x02,             // ADD -> 30
        0x0e,             // DBG -> 30
        0x0f,             // HLT
    ];

    let program2: Vec<u8> = vec![
        0x01, 0x00, 0x00, // PUSH 0 -> 20
        0x06, 0x04, 0x00, // JMP 3
        0x0f,             // HLT
        0x01, 0x01, 0x00, // PUSH 1 -> 20 10
        0x02,             // ADD -> 30
        0x0e,             // DBG -> 30
        0x0f,             // HLT
    ];
    vm.load_program(program2);
    
    let run_result = vm.run();
    match run_result {
        Ok(()) => println!("Все хорошо:)"),
        Err(e) => println!("Ошибка: {e}"),
    }
}
