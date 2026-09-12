use crate::error::CarlaeError;
use crate::expr::{Expr, LiteralValue};
use crate::token::{Token, TokenKind};

type ParserRule = fn(&mut Parser) -> Result<Expr, CarlaeError>;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Expr, CarlaeError> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, CarlaeError> {
        self.not()
    }

    fn not(&mut self) -> Result<Expr, CarlaeError> {
        let expr = if self.current_matches(&[TokenKind::Not]) {
            self.advance();
            let operator = self.previous().clone();
            let right = self.not()?;
            Expr::Unary {
                operator,
                right: Box::new(right),
            }
        } else {
            self.equality()?
        };

        Ok(expr)
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

    fn term(&mut self) -> Result<Expr, CarlaeError> {
        self.parse_binary_operator(Self::factor, &[TokenKind::Minus, TokenKind::Plus])
    }

    fn factor(&mut self) -> Result<Expr, CarlaeError> {
        self.parse_binary_operator(Self::unary, &[TokenKind::Slash, TokenKind::Star])
    }

    fn parse_binary_operator(
        &mut self,
        operand_rule: ParserRule,
        operators: &[TokenKind],
    ) -> Result<Expr, CarlaeError> {
        let mut expr = operand_rule(self)?;

        while self.current_matches(operators) {
            self.advance();
            let operator = self.previous().clone();
            let right = operand_rule(self)?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, CarlaeError> {
        let expr = if self.current_matches(&[TokenKind::Minus]) {
            self.advance();
            let operator = self.previous().clone();
            let right = self.unary()?;
            Expr::Unary {
                operator,
                right: Box::new(right),
            }
        } else {
            self.primary()?
        };

        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expr, CarlaeError> {
        if let Some(t) = self.peek() {
            let val = match &t.kind {
                TokenKind::Number(n) => Expr::Literal(LiteralValue::Number(*n)),
                TokenKind::String(s) => Expr::Literal(LiteralValue::String(s.clone())),
                TokenKind::True => Expr::Literal(LiteralValue::Boolean(true)),
                TokenKind::False => Expr::Literal(LiteralValue::Boolean(false)),
                TokenKind::None => Expr::Literal(LiteralValue::None),
                TokenKind::LeftParen => {
                    self.advance();
                    return self.grouping();
                }
                _ => {
                    return Err(CarlaeError::Parsing(format!(
                        "[Line {}] Invalid token {:?}",
                        t.line, t.kind
                    )));
                }
            };
            self.advance();
            Ok(val)
        } else {
            let prev = self.previous();
            Err(CarlaeError::Parsing(format!(
                "[Line {}] Expected token after {:?}",
                prev.line, prev.kind
            )))
        }
    }

    fn grouping(&mut self) -> Result<Expr, CarlaeError> {
        let expr = self.expression()?;
        if self.peek().is_some_and(|t| t.kind == TokenKind::RightParen) {
            self.advance();
            Ok(Expr::Grouping(Box::new(expr)))
        } else {
            let prev = self.previous();
            Err(CarlaeError::Parsing(format!(
                "[Line {}] Missing `)` after {:?}",
                prev.line, prev.kind
            )))
        }
    }

    fn current_matches(&self, kinds: &[TokenKind]) -> bool {
        self.peek().is_some_and(|t| kinds.contains(&t.kind))
    }

    fn advance(&mut self) {
        if !self.is_at_end() {
            self.current += 1;
        };
    }

    fn is_at_end(&self) -> bool {
        self.peek().expect("Token stream ends with EOF").kind == TokenKind::Eof
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn previous(&self) -> &Token {
        self.current
            .checked_sub(1)
            .and_then(|i| self.tokens.get(i))
            .expect("Previous token exists")
    }

    fn _synchronize(&mut self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_basic_binary_op() {
        let mut parser = Parser::new(vec![
            Token::new(TokenKind::LeftParen, "(".into(), 1),
            Token::new(TokenKind::Number(1.0), "1".into(), 1),
            Token::new(TokenKind::Plus, "+".into(), 1),
            Token::new(TokenKind::Number(2.0), "2".into(), 1),
            Token::new(TokenKind::RightParen, ")".into(), 1),
            Token::new(TokenKind::Newline, "\n".into(), 1),
            Token::new(TokenKind::Eof, "".into(), 2),
        ]);
        let expected = Expr::Grouping(Box::new(Expr::Binary {
            left: Box::new(Expr::Literal(LiteralValue::Number(1.0))),
            operator: Token::new(TokenKind::Plus, "+".into(), 1),
            right: Box::new(Expr::Literal(LiteralValue::Number(2.0))),
        }));

        let expr = parser
            .parse()
            .expect("Tokens successfully parsed into Expr");

        assert_eq!(expr, expected);
    }

    #[test]
    fn parses_mult_with_higher_precedence() {
        let mut parser = Parser::new(vec![
            Token::new(TokenKind::Number(1.0), "1".into(), 1),
            Token::new(TokenKind::Plus, "+".into(), 1),
            Token::new(TokenKind::Number(2.0), "2".into(), 1),
            Token::new(TokenKind::Star, "*".into(), 1),
            Token::new(TokenKind::Number(3.0), "3".into(), 1),
            Token::new(TokenKind::Newline, "\n".into(), 1),
            Token::new(TokenKind::Eof, "".into(), 2),
        ]);
        let expected = Expr::Binary {
            left: Box::new(Expr::Literal(LiteralValue::Number(1.0))),
            operator: Token::new(TokenKind::Plus, "+".into(), 1),
            right: Box::new(Expr::Binary {
                left: Box::new(Expr::Literal(LiteralValue::Number(2.0))),
                operator: Token::new(TokenKind::Star, "*".into(), 1),
                right: Box::new(Expr::Literal(LiteralValue::Number(3.0))),
            }),
        };

        let expr = parser
            .parse()
            .expect("Tokens successfully parsed into Expr");

        assert_eq!(expr, expected);
    }

    #[test]
    fn parses_subtraction_as_left_associative() {
        let mut parser = Parser::new(vec![
            Token::new(TokenKind::Number(8.0), "8".into(), 1),
            Token::new(TokenKind::Minus, "-".into(), 1),
            Token::new(TokenKind::Number(3.0), "3".into(), 1),
            Token::new(TokenKind::Minus, "-".into(), 1),
            Token::new(TokenKind::Number(2.0), "2".into(), 1),
            Token::new(TokenKind::Newline, "\n".into(), 1),
            Token::new(TokenKind::Eof, "".into(), 2),
        ]);
        let expected = Expr::Binary {
            left: Box::new(Expr::Binary {
                left: Box::new(Expr::Literal(LiteralValue::Number(8.0))),
                operator: Token::new(TokenKind::Minus, "-".into(), 1),
                right: Box::new(Expr::Literal(LiteralValue::Number(3.0))),
            }),
            operator: Token::new(TokenKind::Minus, "-".into(), 1),
            right: Box::new(Expr::Literal(LiteralValue::Number(2.0))),
        };

        let expr = parser
            .parse()
            .expect("Tokens successfully parsed into Expr");

        assert_eq!(expr, expected);
    }

    #[test]
    fn parses_nested_unary_with_grouping() {
        let mut parser = Parser::new(vec![
            Token::new(TokenKind::Minus, "-".into(), 1),
            Token::new(TokenKind::Minus, "-".into(), 1),
            Token::new(TokenKind::LeftParen, "(".into(), 1),
            Token::new(TokenKind::Number(1.0), "1".into(), 1),
            Token::new(TokenKind::Plus, "+".into(), 1),
            Token::new(TokenKind::Number(2.0), "2".into(), 1),
            Token::new(TokenKind::RightParen, ")".into(), 1),
            Token::new(TokenKind::Newline, "\n".into(), 1),
            Token::new(TokenKind::Eof, "".into(), 2),
        ]);
        let expected = Expr::Unary {
            operator: Token::new(TokenKind::Minus, "-".into(), 1),
            right: Box::new(Expr::Unary {
                operator: Token::new(TokenKind::Minus, "-".into(), 1),
                right: Box::new(Expr::Grouping(Box::new(Expr::Binary {
                    left: Box::new(Expr::Literal(LiteralValue::Number(1.0))),
                    operator: Token::new(TokenKind::Plus, "+".into(), 1),
                    right: Box::new(Expr::Literal(LiteralValue::Number(2.0))),
                }))),
            }),
        };

        let expr = parser
            .parse()
            .expect("Tokens successfully parsed into Expr");

        assert_eq!(expr, expected);
    }

    #[test]
    fn parses_repeated_not_with_equality() {
        let mut parser = Parser::new(vec![
            Token::new(TokenKind::Not, "not".into(), 1),
            Token::new(TokenKind::Not, "not".into(), 1),
            Token::new(TokenKind::Number(1.0), "1".into(), 1),
            Token::new(TokenKind::EqualEqual, "==".into(), 1),
            Token::new(TokenKind::Number(1.0), "1".into(), 1),
            Token::new(TokenKind::Newline, "\n".into(), 1),
            Token::new(TokenKind::Eof, "".into(), 2),
        ]);
        let expected = Expr::Unary {
            operator: Token::new(TokenKind::Not, "not".into(), 1),
            right: Box::new(Expr::Unary {
                operator: Token::new(TokenKind::Not, "not".into(), 1),
                right: Box::new(Expr::Binary {
                    left: Box::new(Expr::Literal(LiteralValue::Number(1.0))),
                    operator: Token::new(TokenKind::EqualEqual, "==".into(), 1),
                    right: Box::new(Expr::Literal(LiteralValue::Number(1.0))),
                }),
            }),
        };

        let expr = parser
            .parse()
            .expect("Tokens successfully parsed into Expr");

        assert_eq!(expr, expected);
    }

    #[test]
    fn rejects_not_as_right_operand_of_equality() {
        let mut parser = Parser::new(vec![
            Token::new(TokenKind::Number(1.0), "1".into(), 1),
            Token::new(TokenKind::EqualEqual, "==".into(), 1),
            Token::new(TokenKind::Not, "not".into(), 1),
            Token::new(TokenKind::Number(1.0), "1".into(), 1),
            Token::new(TokenKind::Newline, "\n".into(), 1),
            Token::new(TokenKind::Eof, "".into(), 2),
        ]);
        let expected = "invalid token";

        let result = parser.parse();

        assert!(matches!(
            result,
            Err(CarlaeError::Parsing(message))
            if message.to_lowercase().contains(expected)
        ));
    }

    #[test]
    fn rejects_missing_right_paren() {
        let mut parser = Parser::new(vec![
            Token::new(TokenKind::LeftParen, "(".into(), 1),
            Token::new(TokenKind::Number(1.0), "1".into(), 1),
            Token::new(TokenKind::Plus, "+".into(), 1),
            Token::new(TokenKind::Number(2.0), "2".into(), 1),
            Token::new(TokenKind::Newline, "\n".into(), 1),
            Token::new(TokenKind::Eof, "".into(), 2),
        ]);
        let expected = "missing `)`";

        let result = parser.parse();

        assert!(matches!(
            result,
            Err(CarlaeError::Parsing(message))
            if message.to_lowercase().contains(expected)
        ));
    }

    #[test]
    fn rejects_invalid_token_in_primary() {
        let mut parser = Parser::new(vec![
            Token::new(TokenKind::Comma, ",".into(), 1),
            Token::new(TokenKind::Eof, "".into(), 1),
        ]);
        let expected = "invalid token";

        let result = parser.parse();

        assert!(matches!(
            result,
            Err(CarlaeError::Parsing(message))
            if message.to_lowercase().contains(expected)
        ));
    }

    #[test]
    #[should_panic(expected = "Token stream ends with EOF")]
    fn panics_if_token_stream_is_missing_eof() {
        let mut parser = Parser::new(vec![Token::new(TokenKind::Number(1.0), "1".into(), 1)]);

        parser.advance();
        parser.is_at_end();
    }
}
