use std::fmt::Debug;

use crate::error::CarlaeError;
use crate::expr::LiteralValue;
use crate::interpreter::Interpreter;
use crate::stmt::Stmt;
use crate::token::Token;

pub trait Callable: Debug {
    fn call(
        &self,
        args: Vec<LiteralValue>,
        interpreter: &Interpreter,
    ) -> Result<LiteralValue, CarlaeError>;

    fn arity(&self) -> usize;

    fn name(&self) -> &str;
}

#[derive(Debug, PartialEq)]
pub struct CarlaeFunction {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Stmt>,
}

impl Callable for CarlaeFunction {
    fn call(
        &self,
        args: Vec<LiteralValue>,
        interpreter: &Interpreter,
    ) -> Result<LiteralValue, CarlaeError> {
        todo!()
    }

    fn arity(&self) -> usize {
        self.params.len()
    }

    fn name(&self) -> &str {
        &self.name.lexeme
    }
}
