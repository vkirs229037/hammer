mod errors;
mod instruction;
mod vm;

use vm::*;

fn main() {
    let mut vm = VM::new();
    vm.add_const(1f64);
    vm.add_const(3f64);
    let program: Vec<u8> = vec![0x01,       // PUSH
                                0x00, 0x00, // 0x0000 -> 1
                                0x01,       // PUSH
                                0x01, 0x00, // 0x0001 -> 3
                                0x02,       // ADD
                                0x0e,       // DBG -> 4
                                0x0f];      // HLT
    vm.load_program(program);
    
    let run_result = vm.run();
    match run_result {
        Ok(()) => println!("Все хорошо:)"),
        Err(e) => println!("Ошибка: {e}"),
    }
}
