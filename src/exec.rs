use std::rc::Rc;

use crate::callable::CarlaeFunction;
use crate::error::CarlaeError;
use crate::expr::LiteralValue;
use crate::interpreter::Interpreter;
use crate::stmt::{
    Assignment, FuncDef, IfClause, PrintConfig, PrintMode, ReturnValue, Stmt, WhileClause,
};

impl Interpreter {
    pub fn execute(&mut self, stmt: &Stmt) -> Result<(), CarlaeError> {
        match stmt {
            Stmt::Expression(expr) => {
                self.evaluate(expr)?;
                Ok(())
            }
            Stmt::Function(f @ FuncDef { name, .. }) => {
                let func = Rc::new(CarlaeFunction { def: f.clone() });
                self.env
                    .borrow_mut()
                    .bind(name.lexeme.clone(), LiteralValue::Function(func));
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
            }
            Stmt::While(WhileClause { cond, body }) => {
                while self.evaluate(cond)?.is_truthy() {
                    for stmt in body.iter() {
                        self.execute(stmt)?;
                    }
                }
                Ok(())
            }
            Stmt::Print(PrintConfig {
                keyword,
                exprs,
                mode,
            }) => {
                let vals = exprs
                    .iter()
                    .map(|e| self.evaluate(e))
                    .collect::<Result<Vec<LiteralValue>, CarlaeError>>()?;
                if let Some(e) = vals.first() {
                    print!("{e}")
                }
                for e in vals.iter().skip(1) {
                    print!(" {e}")
                }
                if matches!(mode, PrintMode::FinalNewline) {
                    println!();
                }
                Ok(())
            }
            Stmt::Return(ReturnValue { value, .. }) => {
                let ret = if let Some(expr) = value {
                    Some(self.evaluate(expr)?)
                } else {
                    None
                };

                Err(CarlaeError::Return(ret))
            }
            Stmt::Variable(Assignment { name, initializer }) => {
                let value = self.evaluate(initializer)?;
                self.env.borrow_mut().bind(name.lexeme.to_string(), value);
                Ok(())
            }
        }
    }
}
