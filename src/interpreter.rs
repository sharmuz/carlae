use crate::environment::Environment;
use crate::error::CarlaeError;
use crate::stmt::Stmt;

pub struct Interpreter {
    pub env: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            env: Environment::new(),
        }
    }

    pub fn interpret(&mut self, program: &[Stmt]) -> Result<(), CarlaeError> {
        for stmt in program.iter() {
            // TODO: self.execute(stmt)?
        }

        Ok(())
    }

    // TODO: Move from eval.rs and exec.rs to here?
}
