use crate::error::CarlaeError;
use crate::expr::{Expr, LiteralValue};

impl Expr {
    pub fn evaluate(&self) -> Result<LiteralValue, CarlaeError> {
        match self {
            Self::Literal(val) => Ok(val.clone()),
            Self::Unary { operator, right } => todo!(),
            Self::Binary {
                left,
                operator,
                right,
            } => todo!(),
            Self::Grouping(expr) => expr.evaluate(),
        }
    }
}
