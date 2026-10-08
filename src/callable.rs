use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

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
        let previous = interpreter.env.clone();
        let globals = interpreter.globals.borrow().clone();
        interpreter.env = Rc::new(RefCell::new(globals));

        for (param, arg) in self.def.params.iter().zip(args) {
            interpreter.env.borrow_mut().bind(param.lexeme.clone(), arg);
        }

        let res: Result<(), CarlaeError> = self
            .def
            .body
            .iter()
            .try_for_each(|stmt| interpreter.execute(stmt));
        interpreter.env = previous;

        match res {
            // Implicitly returning None if no return stmt
            Ok(_) => Ok(LiteralValue::None),
            Err(e) => Err(e),
        }
    }

    fn arity(&self) -> usize {
        self.def.params.len()
    }

    fn name(&self) -> &str {
        &self.def.name.lexeme
    }
}
