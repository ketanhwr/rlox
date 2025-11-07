use crate::chunk::{Chunk, OpCode, Value};

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub struct VM<'a> {
    pub chunk: &'a Chunk,
    pub ip: usize, // TODO: Change to iterator
    pub stack: Vec<Value>,
}

impl VM<'_> {
    pub fn new(chunk: &Chunk) -> VM<'_> {
        VM {
            chunk,
            ip: 0,
            stack: Vec::with_capacity(256),
        }
    }

    pub fn run(&mut self) -> InterpretResult {
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
                OpCode::Negate => {
                    let value = self.stack.pop().unwrap(); // TODO: Handle None case
                    self.stack.push(-value);
                }
                OpCode::Add | OpCode::Subtract | OpCode::Multiply | OpCode::Divide => {
                    let rhs = self.stack.pop().unwrap();
                    let lhs = self.stack.pop().unwrap();
                    let res = match instr {
                        OpCode::Add => lhs + rhs,
                        OpCode::Subtract => lhs - rhs,
                        OpCode::Multiply => lhs * rhs,
                        OpCode::Divide => lhs / rhs,
                        _ => unreachable!(),
                    };
                    self.stack.push(res);
                }
            }
        }
    }
}
