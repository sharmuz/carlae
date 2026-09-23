use crate::error::CarlaeError;
use crate::interpreter::Interpreter;
use crate::stmt::{Assignment, PrintConfig, PrintMode, Stmt};

impl Interpreter {
    pub fn execute(&mut self, stmt: &Stmt) -> Result<(), CarlaeError> {
        match stmt {
            Stmt::Expression(expr) => {
                self.evaluate(expr)?;
                Ok(())
            }
            Stmt::Print(PrintConfig { exprs, mode }) => {
                if let Some(e) = exprs.first() {
                    print!("{}", self.evaluate(e)?)
                }
                for e in exprs.iter().skip(1) {
                    print!(" {}", self.evaluate(e)?)
                }
                if matches!(mode, PrintMode::FinalNewline) {
                    println!();
                }
                Ok(())
            }
            Stmt::Variable(Assignment { name, initializer }) => {
                let value = self.evaluate(initializer)?;
                self.env.bind(name.lexeme.to_string(), value);
                Ok(())
            }
        }
    }
}
