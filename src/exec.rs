use crate::error::CarlaeError;
use crate::stmt::{PrintConfig, PrintMode, Stmt};

impl Stmt {
    pub fn execute(&self) -> Result<(), CarlaeError> {
        match self {
            Self::Expression(expr) => {
                expr.evaluate()?;
                Ok(())
            }
            Self::Print(PrintConfig { exprs, mode }) => {
                if let Some(e) = exprs.first() {
                    print!("{}", e.evaluate()?)
                }
                for e in exprs.iter().skip(1) {
                    print!(" {}", e.evaluate()?)
                }
                if matches!(mode, PrintMode::FinalNewline) {
                    println!();
                }
                Ok(())
            }
            Self::Variable(_) => todo!(),
        }
    }
}
