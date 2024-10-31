mod errors;
mod instruction;
mod vm;

use vm::*;

fn main() {
    let mut vm = VM::new();
    vm.add_const(10f64);
    vm.add_const(20f64);
    vm.add_const(0f64);
    vm.add_const(1f64);
    let program: Vec<u8> = vec![
        0x01, 0x00, 0x00, // PUSH 0x0000 -> 10
        0x01, 0x01, 0x00, // PUSH 0x0001 -> 10 20
        0x09, 0x08, 0x00, // JG 0x0008 
        0x01, 0x02, 0x00, // PUSH 0x0002 -> 10 20 0
        0x0e,             // DBG -> 0
        0x0f,             // HLT
        0x01, 0x03, 0x00, // PUSH 0x0003 -> 10 20 1
        0x0e,             // DBG -> 1
        0x0f,             // HLT
    ];
    vm.load_program(program);
    
    let run_result = vm.run();
    match run_result {
        Ok(()) => println!("Все хорошо:)"),
        Err(e) => println!("Ошибка: {e}"),
    }
}
