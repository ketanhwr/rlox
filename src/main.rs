#[derive(Debug)]
enum OpCode {
    OpReturn,
    OpConstant(usize),
}

struct Chunk {
    code: Vec<OpCode>,
    lines: Vec<usize>,
    constants: Vec<f64>,
}

impl Chunk {
    fn write_chunk(&mut self, opcode: OpCode, line: usize) {
        self.code.push(opcode);
        self.lines.push(line)
    }

    fn add_constant(&mut self, value: f64) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }
}

fn main() {
    let mut chunk = Chunk {
        code: Vec::new(),
        lines: Vec::new(),
        constants: Vec::new(),
    };

    let constant = OpCode::OpConstant(chunk.add_constant(1.2));
    chunk.write_chunk(constant, 123);
    chunk.write_chunk(OpCode::OpReturn, 123);

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
        match chunk.code[i] {
            OpCode::OpReturn => println!("{:?}", chunk.code[i]),
            OpCode::OpConstant(idx) => println!("{:?} {}", chunk.code[i], chunk.constants[idx]),
        }
    }
}
