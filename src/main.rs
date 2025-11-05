#[derive(Debug)]
enum OpCode {
    Return,
    Constant(usize),
}

type Value = f64;

struct Chunk {
    code: Vec<OpCode>,
    lines: Vec<usize>,
    constants: Vec<Value>,
}

impl Chunk {
    fn write_chunk(&mut self, opcode: OpCode, line: usize) {
        self.code.push(opcode);
        self.lines.push(line)
    }

    fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }
}

enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

struct VM<'a> {
    chunk: &'a Chunk,
    ip: usize,  // TODO: Change to iterator
    stack: Vec<Value>,
}

impl VM<'_> {
    fn new(chunk: &Chunk) -> VM<'_> {
        VM {
            chunk,
            ip: 0,
            stack: Vec::with_capacity(256),
        }
    }

    fn run(&mut self) -> InterpretResult {
        loop {
            let idx = self.ip;
            self.ip += 1;
            let instr = &self.chunk.code[idx];
            println!("{instr:?} : {:?}", self.stack);
            match instr {
                OpCode::Return => {
                    let value = self.stack.pop().unwrap(); // TODO: Handle None case
                    println!("{value}");
                    return InterpretResult::Ok;
                }
                OpCode::Constant(cons) => {
                    let value = self.chunk.constants[*cons];
                    println!("{value}");
                    self.stack.push(value);
                }
            }
        }
    }
}

fn main() {
    let mut chunk = Chunk {
        code: Vec::new(),
        lines: Vec::new(),
        constants: Vec::new(),
    };

    let constant = OpCode::Constant(chunk.add_constant(1.2));
    chunk.write_chunk(constant, 123);
    chunk.write_chunk(OpCode::Return, 123);

    let mut vm = VM::new(&chunk);
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
        match chunk.code[i] {
            OpCode::Return => println!("{:?}", chunk.code[i]),
            OpCode::Constant(idx) => println!("{:?} {}", chunk.code[i], chunk.constants[idx]),
        }
    }
}
