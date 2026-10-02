use std::cell::RefCell;
use std::rc::Rc;

use crate::builtins::Clock;
use crate::environment::Environment;
use crate::error::CarlaeError;
use crate::stmt::Stmt;

pub struct Interpreter {
    globals: Rc<RefCell<Environment>>,
    pub env: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = Rc::new(RefCell::new(Environment::new()));
        let env = globals.clone();

        globals.borrow_mut().bind(
            "clock".into(),
            crate::expr::LiteralValue::Function(Rc::new(Clock {
                name: "clock".into(),
            })),
        );

        Interpreter { globals, env }
    }

    pub fn interpret(&mut self, program: &[Stmt]) -> Result<(), CarlaeError> {
        for stmt in program.iter() {
            self.execute(stmt)?
        }

        Ok(())
    }
}
