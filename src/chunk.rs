use std::fmt;

use crate::value::Value;

pub struct Chunk {
    pub code: Vec<OpCode>,
    pub constants: Vec<Value>,
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
