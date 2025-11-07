use crate::chunk::{Chunk, OpCode};

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
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
