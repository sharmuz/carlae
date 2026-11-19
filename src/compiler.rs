use std::rc::Rc;

use crate::chunk::{Chunk, OpCode};
use crate::error::CarlaeError;
use crate::expr::{Expr, LiteralValue};
use crate::stmt::{Assignment, PrintConfig, PrintMode, Stmt};
use crate::token::{Token, TokenKind};
use crate::value::Value;

pub struct Compiler {
    dump_code: bool,
}

impl Compiler {
    pub fn new(dump_code: bool) -> Self {
        Compiler { dump_code }
    }

    pub fn compile(&self, ast: Vec<Stmt>) -> Result<Chunk, CarlaeError> {
        let mut chunk = Chunk::new();

        for stmt in ast.iter() {
            self.statement(&mut chunk, stmt)?;
        }
        let line = chunk.lines.last().unwrap_or(&0);
        chunk.add_op(OpCode::Return, *line);

        if self.dump_code {
            println!("{chunk}")
        }
        Ok(chunk)
    }

    fn statement(&self, chunk: &mut Chunk, stmt: &Stmt) -> Result<(), CarlaeError> {
        match stmt {
            Stmt::Print(cfg) => self.print_stmt(chunk, cfg),
            Stmt::Expression(expr) => self.expression_stmt(chunk, expr),
            Stmt::Variable(Assignment { name, initializer }) => {
                self.assignment_stmt(chunk, name, initializer)
            }
            s => Err(CarlaeError::Compile(format!("Unknown statement {s}"))),
        }
    }

    fn print_stmt(&self, chunk: &mut Chunk, cfg: &PrintConfig) -> Result<(), CarlaeError> {
        for expr in &cfg.exprs {
            self.expression(chunk, expr)?;
        }
        let n = cfg.exprs.len();
        match cfg.mode {
            PrintMode::FinalNewline => chunk.add_op(OpCode::Println(n), cfg.keyword.line),
            PrintMode::NoNewline => chunk.add_op(OpCode::Print(n), cfg.keyword.line),
        }

        Ok(())
    }

    fn expression_stmt(&self, chunk: &mut Chunk, expr: &Expr) -> Result<(), CarlaeError> {
        self.expression(chunk, expr)?;
        let line = chunk.lines.last().unwrap_or(&0);
        chunk.add_op(OpCode::Pop, *line);
        Ok(())
    }

    fn assignment_stmt(
        &self,
        chunk: &mut Chunk,
        name: &Token,
        initializer: &Expr,
    ) -> Result<(), CarlaeError> {
        self.expression(chunk, initializer)?;
        let idx = chunk.add_constant(Value::String(Rc::new(name.lexeme.to_owned())));
        chunk.add_op(OpCode::AssignGlobal(idx), name.line);
        Ok(())
    }

    fn expression(&self, chunk: &mut Chunk, expr: &Expr) -> Result<(), CarlaeError> {
        match expr {
            Expr::Literal { value, token } => self.literal(chunk, value, token),
            Expr::Unary { operator, right } => self.unary(chunk, operator, right),
            Expr::Binary {
                left,
                operator,
                right,
            } => self.binary(chunk, left, operator, right),
            Expr::Grouping(expr) => self.expression(chunk, expr),
            e => Err(CarlaeError::Compile(format!("Unknown expression {e}"))),
        }
    }

    fn unary(&self, chunk: &mut Chunk, operator: &Token, right: &Expr) -> Result<(), CarlaeError> {
        self.expression(chunk, right)?;

        match &operator.kind {
            TokenKind::Minus => chunk.add_op(OpCode::Negate, operator.line),
            TokenKind::Not => chunk.add_op(OpCode::Not, operator.line),
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
            TokenKind::EqualEqual => chunk.add_op(OpCode::Equal, operator.line),
            TokenKind::BangEqual => {
                chunk.add_op(OpCode::Equal, operator.line);
                chunk.add_op(OpCode::Not, operator.line);
            }
            TokenKind::Greater => chunk.add_op(OpCode::Greater, operator.line),
            TokenKind::GreaterEqual => {
                chunk.add_op(OpCode::Less, operator.line);
                chunk.add_op(OpCode::Not, operator.line);
            }
            TokenKind::Less => chunk.add_op(OpCode::Less, operator.line),
            TokenKind::LessEqual => {
                chunk.add_op(OpCode::Greater, operator.line);
                chunk.add_op(OpCode::Not, operator.line);
            }
            op => {
                return Err(CarlaeError::Compile(format!(
                    "[Line {}]: Invalid binary operator {op:?}.",
                    operator.line
                )));
            }
        };

        Ok(())
    }

    fn literal(
        &self,
        chunk: &mut Chunk,
        val: &LiteralValue,
        token: &Token,
    ) -> Result<(), CarlaeError> {
        match val {
            LiteralValue::Number(n) => self.number(chunk, *n, token),
            LiteralValue::Boolean(true) => {
                chunk.add_op(OpCode::True, token.line);
                Ok(())
            }
            LiteralValue::Boolean(false) => {
                chunk.add_op(OpCode::False, token.line);
                Ok(())
            }
            LiteralValue::None => {
                chunk.add_op(OpCode::None, token.line);
                Ok(())
            }
            LiteralValue::String(s) => self.string(chunk, s, token),
            v => Err(CarlaeError::Compile(format!(
                "[Line {}]: Unknown literal value {v}",
                token.line
            ))),
        }
    }

    fn number(&self, chunk: &mut Chunk, n: f64, token: &Token) -> Result<(), CarlaeError> {
        let idx = chunk.add_constant(Value::Number(n));
        chunk.add_op(OpCode::Constant(idx), token.line);
        Ok(())
    }

    fn string(&self, chunk: &mut Chunk, s: &str, token: &Token) -> Result<(), CarlaeError> {
        let idx = chunk.add_constant(Value::String(Rc::new(s.to_string())));
        chunk.add_op(OpCode::Constant(idx), token.line);
        Ok(())
    }
}
