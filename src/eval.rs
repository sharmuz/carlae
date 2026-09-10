use crate::error::CarlaeError;
use crate::expr::{Expr, LiteralValue};
use crate::token::{Token, TokenKind};

impl Expr {
    pub fn evaluate(&self) -> Result<LiteralValue, CarlaeError> {
        match self {
            Self::Literal(val) => Ok(val.clone()),
            Self::Unary { operator, right } => Self::eval_unary(operator, right),
            Self::Binary {
                left,
                operator,
                right,
            } => todo!(),
            Self::Grouping(expr) => expr.evaluate(),
        }
    }

    fn eval_unary(operator: &Token, right: &Self) -> Result<LiteralValue, CarlaeError> {
        let operand = right.evaluate()?;

        match (&operator.kind, operand) {
            (TokenKind::Minus, LiteralValue::Number(n)) => Ok(LiteralValue::Number(-n)),
            (TokenKind::Minus, LiteralValue::Boolean(true)) => Ok(LiteralValue::Number(-1.0)),
            (TokenKind::Minus, LiteralValue::Boolean(false)) => Ok(LiteralValue::Number(0.0)),
            (TokenKind::Minus, val) => Err(CarlaeError::Evaluation(format!(
                "[Line {}]: Bad operand type for unary -: '{val}'",
                operator.line
            ))),
            (k, _) => Err(CarlaeError::Evaluation(format!(
                "[Line {}]: Invalid unary operator: {k:?}",
                operator.line
            ))),
        }
    }
}
