use crate::error::CarlaeError;
use crate::expr::Expr;
use crate::token::{Token, TokenKind};

type ParserRule = fn(&mut Parser) -> Result<Expr, CarlaeError>;

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
        self.parse_binary_operator(
            Self::comparison,
            &[TokenKind::BangEqual, TokenKind::EqualEqual],
        )
    }

    fn comparison(&mut self) -> Result<Expr, CarlaeError> {
        self.parse_binary_operator(
            Self::term,
            &[
                TokenKind::Greater,
                TokenKind::GreaterEqual,
                TokenKind::Less,
                TokenKind::LessEqual,
            ],
        )
    }

    fn parse_binary_operator(
        &mut self,
        operand_rule: ParserRule,
        operators: &[TokenKind],
    ) -> Result<Expr, CarlaeError> {
        let mut expr = operand_rule(self)?;

        while self.current_matches(operators) {
            let operator = if let Some(t) = self.previous() {
                t.clone()
            } else {
                return Err(CarlaeError::Parsing(format!(
                    "No token found at index {}",
                    self.current - 1
                )));
            };
            let right = operand_rule(self)?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, CarlaeError> {
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
        self.peek().is_some_and(|t| t.kind == TokenKind::Eof)
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn previous(&self) -> Option<&Token> {
        self.tokens.get(self.current - 1)
    }
}
