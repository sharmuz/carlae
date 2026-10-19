use crate::chunk::{Chunk, OpCode};
use crate::error::CarlaeError;
use crate::value::Value;

pub struct Vm<'a> {
    chunk: &'a Chunk,
    ip: usize,
    stack: Vec<Value>,
    debug_trace: bool,
}

impl<'a> Vm<'a> {
    pub fn new(chunk: &'a Chunk, debug_trace: bool) -> Self {
        Vm {
            chunk,
            ip: 0,
            stack: Vec::new(),
            debug_trace,
        }
    }

    fn read_op(&mut self) -> OpCode {
        let op = self.chunk.code[self.ip];
        self.ip += 1;
        op
    }

    pub fn read_constant(&self, index: usize) -> Value {
        self.chunk.constants[index]
    }

    #[allow(clippy::never_loop)]
    pub fn run(&mut self) -> Result<(), CarlaeError> {
        loop {
            let instruction = self.read_op();

            if self.debug_trace {
                self.stack_trace(&instruction);
                println!("{instruction}");
            }

            match instruction {
                OpCode::OpReturn => {
                    if let Some(val) = self.stack.pop() {
                        println!("{val}");
                    }
                    return Ok(());
                }
                OpCode::OpConstant(c) => {
                    let val = self.read_constant(c);
                    self.stack.push(val);
                }
                op => return Err(CarlaeError::Compile(format!("Unknown opcode: {op}"))),
            }
        }

        Ok(())
    }

    pub fn stack_trace(&self, instruction: &OpCode) {
        print!("    ");
        if self.stack.is_empty() {
            println!("[ ]");
            return;
        }
        for val in self.stack.iter() {
            print!("[ {val} ]");
        }
        println!();
    }
}
