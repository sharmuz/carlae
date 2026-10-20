use crate::chunk::{Chunk, OpCode};
use crate::error::CarlaeError;
use crate::expr::{Expr, LiteralValue};
use crate::stmt::Stmt;
use crate::token::{Token, TokenKind};

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
            Expr::Unary { operator, right } => self.unary(chunk, operator, right),
            Expr::Binary {
                left,
                operator,
                right,
            } => self.binary(chunk, left, operator, right),
            e => Err(CarlaeError::Compile(format!("Unknown expression {e}"))),
        }
    }

    fn unary(&self, chunk: &mut Chunk, operator: &Token, right: &Expr) -> Result<(), CarlaeError> {
        self.expression(chunk, right)?;

        match &operator.kind {
            TokenKind::Minus => chunk.add_op(OpCode::Negate, operator.line),
            op => {
                return Err(CarlaeError::Compile(format!(
                    "[Line {}]: Invalid unary operator {op:?}.",
                    operator.line
                )));
            }
        };

        Ok(())
    }

    fn binary(
        &self,
        chunk: &mut Chunk,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<(), CarlaeError> {
        self.expression(chunk, left)?;
        self.expression(chunk, right)?;

        match &operator.kind {
            TokenKind::Plus => chunk.add_op(OpCode::Add, operator.line),
            TokenKind::Minus => chunk.add_op(OpCode::Subtract, operator.line),
            TokenKind::Star => chunk.add_op(OpCode::Multiply, operator.line),
            TokenKind::Slash => chunk.add_op(OpCode::Divide, operator.line),
            op => {
                return Err(CarlaeError::Compile(format!(
                    "[Line {}]: Invalid binary operator {op:?}.",
                    operator.line
                )));
            }
        };

        Ok(())
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
