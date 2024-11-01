mod errors;
mod instruction;
mod vm;

use vm::*;

fn main() {
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
}
