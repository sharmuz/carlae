use crate::error::CarlaeError;
use crate::stmt::Stmt;

impl Stmt {
    pub fn execute(&self) -> Result<(), CarlaeError> {
        match self {
            Self::ExpressionStmt(expr) => {
                expr.evaluate()?;
                Ok(())
            }
            Self::PrintStmt(expr) => {
                println!("{}", expr.evaluate()?);
                Ok(())
            }
        }
    }
}
