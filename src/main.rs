#[derive(Debug)]
enum OpCode {
    OpReturn,
    OpConstant(usize),
}

struct Chunk {
    code: Vec<OpCode>,
    constants: Vec<f64>,
}

impl Chunk {
    fn write_chunk(&mut self, opcode: OpCode) {
        self.code.push(opcode);
    }

    fn add_constant(&mut self, value: f64) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }
}

fn main() {
    let mut chunk = Chunk {
        code: Vec::new(),
        constants: Vec::new(),
    };

    chunk.write_chunk(OpCode::OpReturn);

    let constant = OpCode::OpConstant(chunk.add_constant(1.2));
    chunk.write_chunk(constant);

    disassemble_chunk(&chunk, "test chunk");
}

fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("=== {name} ===");

    for (i, instr) in chunk.code.iter().enumerate() {
        println!("{i:04} {instr:?}");
    }
}
