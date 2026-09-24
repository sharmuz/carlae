use crate::expr::Expr;
use crate::token::Token;

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Expression(Expr),
    If(IfClause),
    While(WhileClause),
    Print(PrintConfig),
    Variable(Assignment),
}

impl std::fmt::Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Expression(expr) => write!(f, "{expr}"),
            Self::If(IfClause { cond, then, r#else }) => {
                writeln!(f, "if {cond}:")?;
                for stmt in then.iter() {
                    writeln!(f, "    {stmt}")?;
                }
                if let Some(stmts) = r#else {
                    writeln!(f, "else:")?;
                    for stmt in stmts.iter() {
                        writeln!(f, "    {stmt}")?;
                    }
                };
                Ok(())
            }
            Self::While(WhileClause { cond, body }) => {
                writeln!(f, "while {cond}:")?;
                for stmt in body.iter() {
                    writeln!(f, "    {stmt}")?;
                };
                Ok(())
            }
            Self::Print(PrintConfig { exprs, mode }) => {
                if let Some(e) = exprs.first() {
                    write!(f, "print {e}")?;
                    for e in exprs.iter().skip(1) {
                        write!(f, ", {e}")?;
                    }
                    if matches!(mode, PrintMode::NoNewline) {
                        write!(f, ",")?;
                    }
                    Ok(())
                } else {
                    write!(f, "print")
                }
            }
            Self::Variable(Assignment { name, initializer }) => {
                write!(f, "{} = {}", name.lexeme, initializer)
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct IfClause {
    pub cond: Expr,
    pub then: Vec<Stmt>,
    pub r#else: Option<Vec<Stmt>>,
}

#[derive(Debug, PartialEq)]
pub struct WhileClause {
    pub cond: Expr,
    pub body: Vec<Stmt>,
}

#[derive(Debug, PartialEq)]
pub struct PrintConfig {
    pub exprs: Vec<Expr>,
    pub mode: PrintMode,
}

#[derive(Debug, PartialEq)]
pub enum PrintMode {
    FinalNewline,
    NoNewline,
}

#[derive(Debug, PartialEq)]
pub struct Assignment {
    pub name: Token,
    pub initializer: Expr,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::LiteralValue;
    use crate::token::TokenKind;

    #[test]
    fn if_statement_displays_else_branch() {
        let if_stmt = Stmt::If(IfClause {
            cond: Expr::Binary {
                left: Box::new(Expr::Variable(Token::new(
                    TokenKind::Identifier("x".into()),
                    "x".into(),
                    1,
                ))),
                operator: Token::new(TokenKind::EqualEqual, "==".into(), 1),
                right: Box::new(Expr::Literal(LiteralValue::Number(0.0))),
            },
            then: vec![Stmt::Expression(Expr::Literal(LiteralValue::Boolean(true)))],
            r#else: Some(vec![Stmt::Expression(Expr::Literal(
                LiteralValue::Boolean(false),
            ))]),
        });
        let expected = String::from("if (== x 0):\n    True\nelse:\n    False\n");

        assert_eq!(if_stmt.to_string(), expected);
    }

    #[test]
    fn print_statement_displays_multiple_exprs() {
        let print_stmt = Stmt::Print(PrintConfig {
            exprs: vec![
                Expr::Literal(LiteralValue::Number(1.0)),
                Expr::Literal(LiteralValue::Number(2.0)),
            ],
            mode: PrintMode::FinalNewline,
        });
        let expected = String::from("print 1, 2");

        assert_eq!(print_stmt.to_string(), expected);
    }

    #[test]
    fn print_statement_displays_empty() {
        let print_stmt = Stmt::Print(PrintConfig {
            exprs: Vec::new(),
            mode: PrintMode::FinalNewline,
        });
        let expected = String::from("print");

        assert_eq!(print_stmt.to_string(), expected);
    }
}
