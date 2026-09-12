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
            (TokenKind::Not, val) => Ok(LiteralValue::Boolean(!val.is_truthy())),
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
            TokenKind::EqualEqual => match (left, right) {
                (LiteralValue::Number(n), b @ LiteralValue::Boolean(_))
                | (b @ LiteralValue::Boolean(_), LiteralValue::Number(n)) => {
                    Ok(LiteralValue::Boolean(n == b.as_number()?))
                }
                (x, y) => Ok(LiteralValue::Boolean(x == y)),
            },
            TokenKind::BangEqual => match (left, right) {
                (LiteralValue::Number(n), b @ LiteralValue::Boolean(_))
                | (b @ LiteralValue::Boolean(_), LiteralValue::Number(n)) => {
                    Ok(LiteralValue::Boolean(n != b.as_number()?))
                }
                (x, y) => Ok(LiteralValue::Boolean(x != y)),
            },
            TokenKind::Greater => match (left, right) {
                (LiteralValue::Number(x), LiteralValue::Number(y)) => {
                    Ok(LiteralValue::Boolean(x > y))
                }
                (LiteralValue::Number(n), b @ LiteralValue::Boolean(_)) => {
                    Ok(LiteralValue::Boolean(n > b.as_number()?))
                }
                (b @ LiteralValue::Boolean(_), LiteralValue::Number(n)) => {
                    Ok(LiteralValue::Boolean(b.as_number()? > n))
                }
                (b @ LiteralValue::Boolean(_), c @ LiteralValue::Boolean(_)) => {
                    Ok(LiteralValue::Boolean(b.as_number()? > c.as_number()?))
                }
                (LiteralValue::String(s), LiteralValue::String(t)) => {
                    Ok(LiteralValue::Boolean(s > t))
                }
                (x, y) => Err(CarlaeError::Evaluation(format!(
                    "[Line {}]: Invalid arguments to binary operator >: {x}, {y}",
                    operator.line
                ))),
            },
            TokenKind::GreaterEqual => match (left, right) {
                (LiteralValue::Number(x), LiteralValue::Number(y)) => {
                    Ok(LiteralValue::Boolean(x >= y))
                }
                (LiteralValue::Number(n), b @ LiteralValue::Boolean(_)) => {
                    Ok(LiteralValue::Boolean(n >= b.as_number()?))
                }
                (b @ LiteralValue::Boolean(_), LiteralValue::Number(n)) => {
                    Ok(LiteralValue::Boolean(b.as_number()? >= n))
                }
                (b @ LiteralValue::Boolean(_), c @ LiteralValue::Boolean(_)) => {
                    Ok(LiteralValue::Boolean(b.as_number()? >= c.as_number()?))
                }
                (LiteralValue::String(s), LiteralValue::String(t)) => {
                    Ok(LiteralValue::Boolean(s >= t))
                }
                (x, y) => Err(CarlaeError::Evaluation(format!(
                    "[Line {}]: Invalid arguments to binary operator >=: {x}, {y}",
                    operator.line
                ))),
            },
            TokenKind::Less => match (left, right) {
                (LiteralValue::Number(x), LiteralValue::Number(y)) => {
                    Ok(LiteralValue::Boolean(x < y))
                }
                (LiteralValue::Number(n), b @ LiteralValue::Boolean(_)) => {
                    Ok(LiteralValue::Boolean(n < b.as_number()?))
                }
                (b @ LiteralValue::Boolean(_), LiteralValue::Number(n)) => {
                    Ok(LiteralValue::Boolean(b.as_number()? < n))
                }
                (b @ LiteralValue::Boolean(_), c @ LiteralValue::Boolean(_)) => {
                    Ok(LiteralValue::Boolean(b.as_number()? < c.as_number()?))
                }
                (LiteralValue::String(s), LiteralValue::String(t)) => {
                    Ok(LiteralValue::Boolean(s < t))
                }
                (x, y) => Err(CarlaeError::Evaluation(format!(
                    "[Line {}]: Invalid arguments to binary operator <: {x}, {y}",
                    operator.line
                ))),
            },
            TokenKind::LessEqual => match (left, right) {
                (LiteralValue::Number(x), LiteralValue::Number(y)) => {
                    Ok(LiteralValue::Boolean(x <= y))
                }
                (LiteralValue::Number(n), b @ LiteralValue::Boolean(_)) => {
                    Ok(LiteralValue::Boolean(n <= b.as_number()?))
                }
                (b @ LiteralValue::Boolean(_), LiteralValue::Number(n)) => {
                    Ok(LiteralValue::Boolean(b.as_number()? <= n))
                }
                (b @ LiteralValue::Boolean(_), c @ LiteralValue::Boolean(_)) => {
                    Ok(LiteralValue::Boolean(b.as_number()? <= c.as_number()?))
                }
                (LiteralValue::String(s), LiteralValue::String(t)) => {
                    Ok(LiteralValue::Boolean(s <= t))
                }
                (x, y) => Err(CarlaeError::Evaluation(format!(
                    "[Line {}]: Invalid arguments to binary operator <=: {x}, {y}",
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

    pub fn is_truthy(&self) -> bool {
        match self {
            Self::Boolean(b) => *b,
            Self::Number(n) => *n != 0.0,
            Self::String(s) => !s.is_empty(),
            Self::None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn number(value: f64) -> Expr {
        Expr::Literal(LiteralValue::Number(value))
    }

    fn boolean(value: bool) -> Expr {
        Expr::Literal(LiteralValue::Boolean(value))
    }

    fn string(value: &str) -> Expr {
        Expr::Literal(LiteralValue::String(value.into()))
    }

    fn unary(kind: TokenKind, lexeme: &str, right: Expr) -> Expr {
        Expr::Unary {
            operator: Token::new(kind, lexeme.into(), 1),
            right: Box::new(right),
        }
    }

    fn binary(left: Expr, kind: TokenKind, lexeme: &str, right: Expr) -> Expr {
        Expr::Binary {
            left: Box::new(left),
            operator: Token::new(kind, lexeme.into(), 1),
            right: Box::new(right),
        }
    }

    #[test]
    fn evals_trivial_arithmetic() {
        let expr = Expr::Grouping(Box::new(binary(
            number(1.0),
            TokenKind::Plus,
            "+",
            number(2.0),
        )));
        let expected = LiteralValue::Number(3.0);

        let eval = expr.evaluate().expect("Expression is evaluated");

        assert_eq!(eval, expected);
    }

    #[test]
    fn evals_number_arithmetic_with_all_binary_ops() {
        let expr = Expr::Grouping(Box::new(binary(
            binary(
                binary(
                    binary(number(8.0), TokenKind::Plus, "+", number(4.0)),
                    TokenKind::Star,
                    "*",
                    number(3.0),
                ),
                TokenKind::Slash,
                "/",
                number(2.0),
            ),
            TokenKind::Minus,
            "-",
            number(5.0),
        )));
        let expected = LiteralValue::Number(13.0);

        let eval = expr.evaluate().expect("Expression is evaluated");

        assert_eq!(eval, expected);
    }

    #[test]
    fn evals_arithmetic_with_numbers_and_bools() {
        let expr = Expr::Grouping(Box::new(binary(
            binary(
                binary(
                    binary(number(10.0), TokenKind::Minus, "-", boolean(true)),
                    TokenKind::Star,
                    "*",
                    binary(boolean(false), TokenKind::Plus, "+", number(2.0)),
                ),
                TokenKind::Slash,
                "/",
                boolean(true),
            ),
            TokenKind::Plus,
            "+",
            binary(boolean(true), TokenKind::Star, "*", boolean(false)),
        )));
        let expected = LiteralValue::Number(18.0);

        let eval = expr.evaluate().expect("Expression is evaluated");

        assert_eq!(eval, expected);
    }

    #[test]
    fn evals_string_arithmetic() {
        let expr = Expr::Grouping(Box::new(binary(
            binary(
                binary(
                    binary(string("Car"), TokenKind::Plus, "+", string("lae")),
                    TokenKind::Star,
                    "*",
                    number(2.0),
                ),
                TokenKind::Plus,
                "+",
                binary(string("!"), TokenKind::Star, "*", boolean(true)),
            ),
            TokenKind::Plus,
            "+",
            binary(boolean(false), TokenKind::Star, "*", string("unused")),
        )));
        let expected = LiteralValue::String("CarlaeCarlae!".into());

        let eval = expr.evaluate().expect("Expression is evaluated");

        assert_eq!(eval, expected);
    }

    #[test]
    fn evals_equality_between_numbers() {
        let expr = Expr::Grouping(Box::new(binary(
            binary(number(1.0), TokenKind::Plus, "+", number(2.0)),
            TokenKind::EqualEqual,
            "==",
            binary(number(6.0), TokenKind::Slash, "/", number(2.0)),
        )));
        let expected = LiteralValue::Boolean(true);

        let eval = expr.evaluate().expect("Expression is evaluated");

        assert_eq!(eval, expected);
    }

    #[test]
    fn evals_greater_than_between_strings() {
        let expr = Expr::Grouping(Box::new(binary(
            string("cat"),
            TokenKind::Greater,
            ">",
            string("car"),
        )));
        let expected = LiteralValue::Boolean(true);

        let eval = expr.evaluate().expect("Expression is evaluated");

        assert_eq!(eval, expected);
    }

    #[test]
    fn evals_not_per_truthiness() {
        let cases = [
            (boolean(true), LiteralValue::Boolean(false)),
            (boolean(false), LiteralValue::Boolean(true)),
            (number(0.0), LiteralValue::Boolean(true)),
            (number(-88.0), LiteralValue::Boolean(false)),
            (string(""), LiteralValue::Boolean(true)),
            (string("hello"), LiteralValue::Boolean(false)),
            (
                Expr::Literal(LiteralValue::None),
                LiteralValue::Boolean(true),
            ),
        ];

        for (operand, expected) in cases {
            let expr = unary(TokenKind::Not, "not", operand);

            let eval = expr.evaluate().expect("Expression is evaluated");

            assert_eq!(eval, expected);
        }
    }

    #[test]
    fn evals_repeated_not() {
        let expr = unary(
            TokenKind::Not,
            "not",
            unary(TokenKind::Not, "not", string("hello")),
        );
        let expected = LiteralValue::Boolean(true);

        let eval = expr.evaluate().expect("Expression is evaluated");

        assert_eq!(eval, expected);
    }

    #[test]
    fn refuses_divide_by_zero() {
        let expr = Expr::Grouping(Box::new(binary(
            number(1.0),
            TokenKind::Slash,
            "/",
            number(0.0),
        )));
        let expected = "divide by zero";

        let result = expr.evaluate();

        assert!(matches!(
            result,
            Err(CarlaeError::Evaluation(message))
            if message.to_lowercase().contains(expected)
        ));
    }

    #[test]
    fn refuses_divide_by_false() {
        let expr = Expr::Grouping(Box::new(binary(
            number(1.0),
            TokenKind::Slash,
            "/",
            boolean(false),
        )));
        let expected = "divide by false";

        let result = expr.evaluate();

        assert!(matches!(
            result,
            Err(CarlaeError::Evaluation(message))
            if message.to_lowercase().contains(expected)
        ));
    }

    #[test]
    fn refuses_string_multiplication_by_fraction() {
        let expr = Expr::Grouping(Box::new(binary(
            string("Carlae"),
            TokenKind::Star,
            "*",
            number(1.5),
        )));
        let expected = "fractional numbers";

        let result = expr.evaluate();

        assert!(matches!(
            result,
            Err(CarlaeError::Evaluation(message))
            if message.to_lowercase().contains(expected)
        ));
    }
}
