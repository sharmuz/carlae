use crate::expr::Expr;

#[derive(Debug, PartialEq)]
pub enum Stmt {
    ExpressionStmt(Expr),
    PrintStmt(Vec<Expr>),
}

impl std::fmt::Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExpressionStmt(expr) => write!(f, "{expr}"),
            Self::PrintStmt(exprs) => {
                if let Some(e) = exprs.first() {
                    write!(f, "print {e}")?;
                    for e in exprs.iter().skip(1) {
                        write!(f, ", {e}")?;
                    }
                    Ok(())
                } else {
                    write!(f, "print")
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::expr::LiteralValue;

    use super::*;

    #[test]
    fn print_statement_displays_multiple_exprs() {
        let print_stmt = Stmt::PrintStmt(vec![
            Expr::Literal(LiteralValue::Number(1.0)),
            Expr::Literal(LiteralValue::Number(2.0)),
        ]);
        let expected = String::from("print 1, 2");

        assert_eq!(print_stmt.to_string(), expected);
    }

    #[test]
    fn print_statement_displays_empty() {
        let print_stmt = Stmt::PrintStmt(Vec::new());
        let expected = String::from("print");

        assert_eq!(print_stmt.to_string(), expected);
    }
}
