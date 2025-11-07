mod chunk;
mod debug;
mod vm;

use crate::chunk::{Chunk, OpCode};

fn main() {
    let mut chunk = Chunk {
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

    debug::disassemble_chunk(&chunk, "test chunk");
}
