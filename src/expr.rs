use crate::token::Token;

pub enum Expr {
    Literal(LiteralValue),
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping(Box<Expr>),
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(lit) => write!(f, "{lit}"),
            Self::Unary { operator, right } => write!(f, "({} {})", operator.lexeme, right),
            Self::Binary {
                operator,
                left,
                right,
            } => write!(f, "({} {} {})", operator.lexeme, left, right),
            Self::Grouping(expr) => write! {f, "(group {expr})"},
        }
    }
}

pub enum LiteralValue {
    Number(f64),
    Boolean(bool),
    String(String),
    None,
}

impl std::fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{n}"),
            Self::Boolean(true) => write!(f, "True"),
            Self::Boolean(false) => write!(f, "False"),
            Self::String(s) => write!(f, "{s}"),
            Self::None => write!(f, "None"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::TokenKind;

    #[test]
    fn nested_expr_prints() {
        let operator = Token {
            kind: TokenKind::Plus,
            lexeme: "+".into(),
            line: 1,
        };
        let left = Expr::Literal(LiteralValue::Number(1.0));
        let right = Expr::Literal(LiteralValue::Number(2.0));
        let expr = Expr::Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        };
        let expected = "(+ 1 2)".to_string();

        let printed = expr.to_string();

        assert_eq!(printed, expected)
    }
}
