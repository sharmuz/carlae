use std::fmt;

use crate::value::Value;

pub struct Chunk {
    pub code: Vec<OpCode>,
    constants: Vec<Value>,
    lines: Vec<usize>,
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
                OpCode::OpReturn => writeln!(f, "{op}")?,
                OpCode::OpConstant(i) => writeln!(f, "{op} {}", self.constants[*i])?,
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
