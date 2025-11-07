// This should ideally be repr(u8)
// to mimic C enums / memory efficient
// bytecodes, but this is more of a learning
// exercise for idiomatic Rust
#[derive(Debug)]
pub enum OpCode {
    Return,
    Constant(usize),
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub type Value = f64;

pub struct Chunk {
    pub code: Vec<OpCode>,
    pub lines: Vec<usize>,
    pub constants: Vec<Value>,
}

impl Chunk {
    pub fn write_chunk(&mut self, opcode: OpCode, line: usize) {
        self.code.push(opcode);
        self.lines.push(line)
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }
}
