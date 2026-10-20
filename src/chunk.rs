use std::fmt;

use crate::value::Value;

pub struct Chunk {
    pub code: Vec<OpCode>,
    pub constants: Vec<Value>,
    pub lines: Vec<usize>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn add_op(&mut self, op: OpCode, line: usize) {
        self.code.push(op);
        self.lines.push(line)
    }

    pub fn add_constant(&mut self, constant: Value) {
        self.constants.push(constant)
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, op) in self.code.iter().enumerate() {
            write!(f, "{i:04}")?;
            if i == 0 || self.lines[i] != self.lines[i - 1] {
                write!(f, "  {:>4}  ", self.lines[i])?
            } else {
                write!(f, "  {:>4}  ", '|')?
            }

            match op {
                OpCode::Constant(i) => writeln!(f, "{op} {}", self.constants[*i])?,
                OpCode::Negate | OpCode::Add | OpCode::Return => writeln!(f, "{op}")?,
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy)]
pub enum OpCode {
    Constant(usize),
    Negate,
    Add,
    Return,
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Constant(i) => write!(f, "OP_CONSTANT    {i}"),
            Self::Negate => write!(f, "OP_NEGATE"),
            Self::Add => write!(f, "OP_ADD"),
            Self::Return => write!(f, "OP_RETURN"),
        }
    }
}
