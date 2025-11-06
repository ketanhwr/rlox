mod chunk;
mod vm;

use crate::chunk::{OpCode, Chunk};

fn main() {
    let mut chunk = chunk::Chunk {
        code: Vec::new(),
        lines: Vec::new(),
        constants: Vec::new(),
    };

    // return (-((1.2 + 3.4) / 5.6))
    let mut constant = OpCode::Constant(chunk.add_constant(1.2));
    chunk.write_chunk(constant, 123);

    constant = OpCode::Constant(chunk.add_constant(3.4));
    chunk.write_chunk(constant, 123);

    chunk.write_chunk(OpCode::Add, 123);

    constant = OpCode::Constant(chunk.add_constant(5.6));
    chunk.write_chunk(constant, 123);

    chunk.write_chunk(OpCode::Divide, 123);
    chunk.write_chunk(OpCode::Negate, 123);

    chunk.write_chunk(OpCode::Return, 123);

    let mut vm = vm::VM::new(&chunk);
    vm.run();

    disassemble_chunk(&chunk, "test chunk");
}

fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("=== {name} ===");

    for i in 0..chunk.code.len() {
        print!("{i:04} ");
        if i > 0 && chunk.lines[i] == chunk.lines[i - 1] {
            print!("   | ");
        } else {
            print!("{:4} ", chunk.lines[i]);
        }
        print!("{:?} ", chunk.code[i]);
        match chunk.code[i] {
            OpCode::Constant(idx) => println!("{}", chunk.constants[idx]),
            _ => println!(),
        }
    }
}
