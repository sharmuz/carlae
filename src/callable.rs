use std::fmt::Debug;

use crate::error::CarlaeError;
use crate::expr::LiteralValue;
use crate::interpreter::Interpreter;
use crate::stmt::FuncDef;

pub trait Callable: Debug {
    fn call(
        &self,
        args: Vec<LiteralValue>,
        interpreter: &mut Interpreter,
    ) -> Result<LiteralValue, CarlaeError>;

    fn arity(&self) -> usize;

    fn name(&self) -> &str;
}

#[derive(Debug, PartialEq)]
pub struct CarlaeFunction {
    pub def: FuncDef,
}

impl Callable for CarlaeFunction {
    fn call(
        &self,
        args: Vec<LiteralValue>,
        interpreter: &mut Interpreter,
    ) -> Result<LiteralValue, CarlaeError> {
        todo!()
    }

    fn arity(&self) -> usize {
        self.def.params.len()
    }

    fn name(&self) -> &str {
        &self.def.name.lexeme
    }
}
