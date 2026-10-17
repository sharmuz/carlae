use crate::chunk::{Chunk, OpCode};

pub struct Vm<'a> {
    chunk: &'a Chunk,
    ip: &'a OpCode,
}

impl<'a> Vm<'a> {
    pub fn new(chunk: &'a Chunk) -> Self {
        Vm {
            chunk,
            ip: &chunk.code[0],
        }
    }
}
