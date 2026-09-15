use crate::expr::Expr;

#[derive(Debug, PartialEq)]
pub enum Stmt {
    ExpressionStmt(Expr),
    PrintStmt(Expr),
}

impl std::fmt::Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExpressionStmt(expr) => write!(f, "{expr}"),
            Self::PrintStmt(expr) => write!(f, "print {expr}"),
        }
    }
}
