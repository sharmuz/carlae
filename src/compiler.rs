use crate::chunk::{Chunk, OpCode};
use crate::error::CarlaeError;
use crate::expr::{Expr, LiteralValue};
use crate::stmt::Stmt;

pub struct Compiler;

impl Compiler {
    pub fn new() -> Self {
        Compiler {}
    }

    pub fn compile(&self, ast: Vec<Stmt>) -> Result<Chunk, CarlaeError> {
        let mut chunk = Chunk::new();

        for stmt in ast.iter() {
            self.statement(&mut chunk, stmt);
        }

        chunk.add_op(OpCode::Return, 0);
        Ok(chunk)
    }

    fn statement(&self, chunk: &mut Chunk, stmt: &Stmt) -> Result<(), CarlaeError> {
        match stmt {
            Stmt::Expression(expr) => self.expression(chunk, expr),
            s => Err(CarlaeError::Compile(format!("Unknown statement {s}"))),
        }
    }

    fn expression(&self, chunk: &mut Chunk, expr: &Expr) -> Result<(), CarlaeError> {
        match expr {
            Expr::Literal(val) => self.literal(chunk, val),
            e => Err(CarlaeError::Compile(format!("Unknown expression {e}"))),
        }
    }

    fn literal(&self, chunk: &mut Chunk, val: &LiteralValue) -> Result<(), CarlaeError> {
        match val {
            LiteralValue::Number(n) => self.number(chunk, *n),
            v => Err(CarlaeError::Compile(format!("Unknown literal value {v}"))),
        }
    }

    fn number(&self, chunk: &mut Chunk, n: f64) -> Result<(), CarlaeError> {
        chunk.add_constant(n);
        let idx = chunk.constants.len() - 1;
        chunk.add_op(OpCode::Constant(idx), 0); // TODO: Get actual line number
        Ok(())
    }
}
