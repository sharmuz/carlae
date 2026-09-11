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
            } => Self::eval_binary(left, operator, right),
            Self::Grouping(expr) => expr.evaluate(),
        }
    }

    fn eval_unary(operator: &Token, right: &Self) -> Result<LiteralValue, CarlaeError> {
        let operand = right.evaluate()?;

        match (&operator.kind, operand) {
            (TokenKind::Minus, LiteralValue::Number(n)) => Ok(LiteralValue::Number(-n)),
            (TokenKind::Minus, b @ LiteralValue::Boolean(_)) => {
                Ok(LiteralValue::Number(-b.as_number()?))
            }
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

    fn eval_binary(
        left: &Self,
        operator: &Token,
        right: &Self,
    ) -> Result<LiteralValue, CarlaeError> {
        let left = left.evaluate()?;
        let right = right.evaluate()?;

        match &operator.kind {
            TokenKind::Plus => match (left, right) {
                (LiteralValue::Number(x), LiteralValue::Number(y)) => {
                    Ok(LiteralValue::Number(x + y))
                }
                (LiteralValue::Number(x), b @ LiteralValue::Boolean(_))
                | (b @ LiteralValue::Boolean(_), LiteralValue::Number(x)) => {
                    Ok(LiteralValue::Number(x + b.as_number()?))
                }
                (b @ LiteralValue::Boolean(_), c @ LiteralValue::Boolean(_)) => {
                    Ok(LiteralValue::Number(b.as_number()? + c.as_number()?))
                }
                (LiteralValue::String(s), LiteralValue::String(t)) => {
                    Ok(LiteralValue::String(format!("{s}{t}")))
                }
                (x, y) => Err(CarlaeError::Evaluation(format!(
                    "[Line {}]: Invalid arguments to binary operator +: {x}, {y}",
                    operator.line
                ))),
            },
            TokenKind::Minus => match (left, right) {
                (LiteralValue::Number(x), LiteralValue::Number(y)) => {
                    Ok(LiteralValue::Number(x - y))
                }
                (LiteralValue::Number(x), b @ LiteralValue::Boolean(_)) => {
                    Ok(LiteralValue::Number(x - b.as_number()?))
                }
                (b @ LiteralValue::Boolean(_), LiteralValue::Number(x)) => {
                    Ok(LiteralValue::Number(b.as_number()? - x))
                }
                (b @ LiteralValue::Boolean(_), c @ LiteralValue::Boolean(_)) => {
                    Ok(LiteralValue::Number(b.as_number()? - c.as_number()?))
                }
                (x, y) => Err(CarlaeError::Evaluation(format!(
                    "[Line {}]: Invalid arguments to binary operator -: {x}, {y}",
                    operator.line
                ))),
            },
            TokenKind::Star => match (left, right) {
                (LiteralValue::Number(x), LiteralValue::Number(y)) => {
                    Ok(LiteralValue::Number(x * y))
                }
                (LiteralValue::Number(x), b @ LiteralValue::Boolean(_))
                | (b @ LiteralValue::Boolean(_), LiteralValue::Number(x)) => {
                    Ok(LiteralValue::Number(x * b.as_number()?))
                }
                (b @ LiteralValue::Boolean(_), c @ LiteralValue::Boolean(_)) => {
                    Ok(LiteralValue::Number(b.as_number()? * c.as_number()?))
                }
                (LiteralValue::String(s), LiteralValue::Number(n))
                | (LiteralValue::Number(n), LiteralValue::String(s)) => {
                    if n.fract() == 0.0 {
                        Ok(LiteralValue::String(s.repeat(n as usize)))
                    } else {
                        Err(CarlaeError::Evaluation(format!(
                            "[Line {}]: Can't multiply strings by fractional numbers",
                            operator.line
                        )))
                    }
                }
                (LiteralValue::String(s), b @ LiteralValue::Boolean(_))
                | (b @ LiteralValue::Boolean(_), LiteralValue::String(s)) => {
                    Ok(LiteralValue::String(s.repeat(b.as_number()? as usize)))
                }
                (x, y) => Err(CarlaeError::Evaluation(format!(
                    "[Line {}]: Invalid arguments to binary operator *: {x}, {y}",
                    operator.line
                ))),
            },
            TokenKind::Slash => match (left, right) {
                (LiteralValue::Number(x), LiteralValue::Number(y)) => {
                    if y == 0.0 {
                        Err(CarlaeError::Evaluation(format!(
                            "[Line {}]: Cannot divide by zero",
                            operator.line
                        )))
                    } else {
                        Ok(LiteralValue::Number(x / y))
                    }
                }
                (LiteralValue::Number(x), b @ LiteralValue::Boolean(p)) => {
                    if !p {
                        Err(CarlaeError::Evaluation(format!(
                            "[Line {}]: Cannot divide by {b} == zero",
                            operator.line
                        )))
                    } else {
                        Ok(LiteralValue::Number(x / b.as_number()?))
                    }
                }
                (b @ LiteralValue::Boolean(_), LiteralValue::Number(x)) => {
                    if x == 0.0 {
                        Err(CarlaeError::Evaluation(format!(
                            "[Line {}]: Cannot divide by zero",
                            operator.line
                        )))
                    } else {
                        Ok(LiteralValue::Number(b.as_number()? / x))
                    }
                }
                (b @ LiteralValue::Boolean(_), c @ LiteralValue::Boolean(p)) => {
                    if !p {
                        Err(CarlaeError::Evaluation(format!(
                            "[Line {}]: Cannot divide by {c} == zero",
                            operator.line
                        )))
                    } else {
                        Ok(LiteralValue::Number(b.as_number()? / c.as_number()?))
                    }
                }
                (x, y) => Err(CarlaeError::Evaluation(format!(
                    "[Line {}]: Invalid arguments to binary operator /: {x}, {y}",
                    operator.line
                ))),
            },
            k => Err(CarlaeError::Evaluation(format!(
                "[Line {}]: Invalid binary operator: {k:?}",
                operator.line
            ))),
        }
    }
}

impl LiteralValue {
    pub fn as_number(&self) -> Result<f64, CarlaeError> {
        match self {
            Self::Number(n) => Ok(*n),
            Self::Boolean(b) => Ok((*b as i32) as f64),
            Self::String(s) => Err(CarlaeError::Evaluation(format!(
                "Cannot convert String to f64: {s}"
            ))),
            Self::None => Err(CarlaeError::Evaluation("Cannot convert None to f64".into())),
        }
    }
}
