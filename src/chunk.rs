use std::fmt;

pub struct Chunk {
    pub code: Vec<OpCode>,
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, op) in self.code.iter().enumerate() {
            writeln!(f, "{i:04} {op}")?;
        }
        Ok(())
    }
}

pub enum OpCode {
    OpReturn,
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OpReturn => write!(f, "OP_RETURN"),
        }
    }
}
