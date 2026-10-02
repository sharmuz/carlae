use std::rc::Rc;

use crate::callable::Callable;
use crate::token::Token;

#[derive(Clone, Debug, PartialEq)]
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
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping(Box<Expr>),
    Variable(Token),
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
        paren: Token,
    },
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
            }
            | Self::Logical {
                operator,
                left,
                right,
            } => write!(f, "({} {} {})", operator.lexeme, left, right),
            Self::Grouping(expr) => write!(f, "(group {expr})"),
            Self::Variable(t) => write!(f, "{}", t.lexeme),
            Self::Call { callee, args, .. } => {
                write!(f, "{callee}(")?;
                let mut xs = args.iter();
                if let Some(x) = xs.next() {
                    write!(f, "{x}")?;
                    for y in xs {
                        write!(f, ", {y}")?;
                    }
                };
                write!(f, ")")
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum LiteralValue {
    Number(f64),
    Boolean(bool),
    String(String),
    Function(Rc<dyn Callable>),
    None,
}

impl std::fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{n}"),
            Self::Boolean(true) => write!(f, "True"),
            Self::Boolean(false) => write!(f, "False"),
            Self::String(s) => write!(f, "{s}"),
            Self::Function(g) => write!(f, "{}", g.name()),
            Self::None => write!(f, "None"),
        }
    }
}

impl PartialEq for LiteralValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(x), Self::Number(y)) => x == y,
            (Self::Boolean(p), Self::Boolean(q)) => p == q,
            (Self::String(s), Self::String(t)) => s == t,
            (Self::Function(f), Self::Function(g)) => Rc::ptr_eq(f, g),
            (Self::None, Self::None) => true,
            (_, _) => false,
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
