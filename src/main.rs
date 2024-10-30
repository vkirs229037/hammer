type Value = f64;

enum Instruction {
    PUSH(Value),
    ADD,
    SUB,
    MUL,
    DIV,
}


struct VM {
    stack: Vec<Value>,
    program: Vec<Instruction>,
}

fn main() {
    println!("Hello, world!");
}
