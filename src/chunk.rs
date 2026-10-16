use std::fmt;

use crate::value::Value;

pub struct Chunk {
    code: Vec<OpCode>,
    constants: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
        }
    }

    pub fn add_op(&mut self, op: OpCode) {
        self.code.push(op)
    }

    pub fn add_constant(&mut self, constant: Value) {
        self.constants.push(constant)
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, op) in self.code.iter().enumerate() {
            match op {
                OpCode::OpReturn => writeln!(f, "{i:04} {op}")?,
                OpCode::OpConstant(i) => writeln!(f, "{i:04} {op} {}", self.constants[*i])?,
            }
        }
        Ok(())
    }
}

pub enum OpCode {
    OpConstant(usize),
    OpReturn,
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OpReturn => write!(f, "OP_RETURN"),
            Self::OpConstant(i) => write!(f, "OP_CONSTANT    {i}"),
        }
    }
}
