use crate::error::CarlaeError;
use crate::expr::LiteralValue;
use crate::interpreter::Interpreter;
use crate::stmt::Stmt;
use crate::token::Token;

pub trait Callable {
    fn call(
        &self,
        args: Vec<LiteralValue>,
        interpreter: &Interpreter,
    ) -> Result<LiteralValue, CarlaeError>;
}

#[derive(Clone, Debug, PartialEq)]
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
}
