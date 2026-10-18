use crate::chunk::{Chunk, OpCode};
use crate::error::CarlaeError;

pub struct Vm<'a> {
    chunk: &'a Chunk,
    ip: usize,
}

impl<'a> Vm<'a> {
    pub fn new(chunk: &'a Chunk) -> Self {
        Vm { chunk, ip: 0 }
    }

    fn read_op(&mut self) -> &'_ OpCode {
        let op = &self.chunk.code[self.ip];
        self.ip += 1;
        op
    }

    #[allow(clippy::never_loop)]
    pub fn run(&mut self) -> Result<(), CarlaeError> {
        loop {
            let instruction = self.read_op();
            match instruction {
                OpCode::OpReturn => return Ok(()),
                op => return Err(CarlaeError::Compile(format!("Unknown opcode {op}"))),
            }
        }
    }
}
