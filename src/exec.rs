use crate::error::CarlaeError;
use crate::stmt::Stmt;

impl Stmt {
    pub fn execute(&self) -> Result<(), CarlaeError> {
        match self {
            Self::ExpressionStmt(expr) => {
                expr.evaluate()?;
                Ok(())
            }
            Self::PrintStmt(exprs) => {
                if let Some(e) = exprs.first() {
                    print!("{}", e.evaluate()?)
                }
                for e in exprs.iter().skip(1) {
                    print!(" {}", e.evaluate()?)
                }
                println!();
                Ok(())
            }
        }
    }
}
