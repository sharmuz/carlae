use crate::expr::Expr;

pub enum Stmt {
    ExpressionStmt(Expr),
    PrintStmt(Expr),
}
