use crate::error::CarlaeError;
use crate::interpreter::Interpreter;
use crate::stmt::{Assignment, IfClause, PrintConfig, PrintMode, Stmt, WhileClause};

impl Interpreter {
    pub fn execute(&mut self, stmt: &Stmt) -> Result<(), CarlaeError> {
        match stmt {
            Stmt::Expression(expr) => {
                self.evaluate(expr)?;
                Ok(())
            }
            Stmt::If(IfClause { cond, then, r#else }) => {
                if self.evaluate(cond)?.is_truthy() {
                    for stmt in then.iter() {
                        self.execute(stmt)?;
                    }
                } else if let Some(else_stmts) = r#else {
                    for stmt in else_stmts.iter() {
                        self.execute(stmt)?;
                    }
                }
                Ok(())
            },
            Stmt::While(WhileClause { cond, body }) => {
                while self.evaluate(cond)?.is_truthy() {
                    for stmt in body.iter() {
                        self.execute(stmt)?;
                    }
                }
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
