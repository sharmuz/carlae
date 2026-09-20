use crate::expr::Expr;
use crate::token::Token;

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Expression(Expr),
    Print(PrintConfig),
    Variable(VariableDeclaration),
}

impl std::fmt::Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Expression(expr) => write!(f, "{expr}"),
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
            Self::Variable(VariableDeclaration { name, initializer }) => {
                write!(f, "{} = {}", name.lexeme, initializer)
            }
        }
    }
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
pub struct VariableDeclaration {
    pub name: Token,
    pub initializer: Expr,
}

#[cfg(test)]
mod tests {
    use crate::expr::LiteralValue;

    use super::*;

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
