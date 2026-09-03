use crate::error::CarlaeError;
use crate::expr::Expr;
use crate::token::{Token, TokenKind};

#[derive(Debug, Default)]
struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn expression(&mut self) -> Result<Expr, CarlaeError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, CarlaeError> {
        todo!()
    }

    fn comparison(&self) -> Result<Expr, CarlaeError> {
        todo!()
    }

    fn current_matches(&mut self, kinds: &[TokenKind]) -> bool {
        let found = kinds.iter().any(|t| self.check(t));
        if found {
            self.advance();
        }
        found
    }

    fn check(&self, kind: &TokenKind) -> bool {
        !self.is_at_end() && self.peek().is_some_and(|t| t.kind == *kind)
    }

    fn advance(&mut self) -> Option<&Token> {
        if !self.is_at_end() {
            self.current += 1;
        };
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().is_some_and(|t| t.kind != TokenKind::Eof)
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current - 1)
    }

    fn previous(&self) -> Option<&Token> {
        self.tokens.get(self.current - 1)
    }
}
