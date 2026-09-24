use crate::error::CarlaeError;
use crate::expr::{Expr, LiteralValue};
use crate::stmt::{Assignment, IfClause, PrintConfig, PrintMode, Stmt, WhileClause};
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

    pub fn parse(&mut self) -> Result<Vec<Stmt>, CarlaeError> {
        self.program()
    }

    fn program(&mut self) -> Result<Vec<Stmt>, CarlaeError> {
        let mut program: Vec<Stmt> = Vec::new();
        while !self.is_at_end() {
            match self.statement() {
                Ok(stmt) => program.push(stmt),
                Err(_) => match self.synchronize() {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                },
            }
        }

        Ok(program)
    }

    fn statement(&mut self) -> Result<Stmt, CarlaeError> {
        if self.current_matches(&[TokenKind::If, TokenKind::While]) {
            self.compound_statement()
        } else {
            self.simple_statement()
        }
    }

    fn compound_statement(&mut self) -> Result<Stmt, CarlaeError> {
        if let Some(t) = self.peek() {
            let line = t.line;
            let stmt = match &t.kind {
                TokenKind::If => {
                    self.advance();
                    self.if_stmt(line)?
                }
                TokenKind::While => {
                    self.advance();
                    self.while_stmt(line)?
                }
                _ => self.expression_stmt(line)?,
            };
            Ok(stmt)
        } else {
            let prev = self.previous();
            Err(CarlaeError::Parsing(format!(
                "[Line {}] Expected token after {:?}",
                prev.line, prev.kind
            )))
        }
    }

    fn if_stmt(&mut self, line: usize) -> Result<Stmt, CarlaeError> {
        let cond = self.expression()?;
        if !self.current_matches(&[TokenKind::Colon]) {
            return Err(CarlaeError::Parsing(format!(
                "[Line {line}] If statement missing `:` after condition",
            )));
        };
        self.advance();
        let then = self.suite()?;

        let mut r#else = None;
        if self.current_matches(&[TokenKind::Else]) {
            self.advance();
            if !self.current_matches(&[TokenKind::Colon]) {
                return Err(CarlaeError::Parsing(format!(
                    "[Line {line}] If statement missing `:` after else",
                )));
            };
            self.advance();
            r#else = Some(self.suite()?);
        }

        Ok(Stmt::If(IfClause { cond, then, r#else }))
    }

    fn while_stmt(&mut self, line: usize) -> Result<Stmt, CarlaeError> {
        let cond = self.expression()?;
        if !self.current_matches(&[TokenKind::Colon]) {
            return Err(CarlaeError::Parsing(format!(
                "[Line {line}] While statement missing `:` after condition",
            )));
        };
        self.advance();
        let body = self.suite()?;

        Ok(Stmt::While(WhileClause { cond, body }))
    }

    fn suite(&mut self) -> Result<Vec<Stmt>, CarlaeError> {
        let mut stmts = Vec::new();
        // We have two possibilities:
        // a) NEWLINE INDENT (statement NEWLINE)+ DEDENT
        // b) simple_statement NEWLINE
        // DELIBERATELY NOT SUPPORTING SEMICOLONS
        if self.current_matches(&[TokenKind::Newline]) && self.next_matches(&[TokenKind::Indent]) {
            self.advance();
            self.advance();
            while !self.current_matches(&[TokenKind::Dedent]) {
                stmts.push(self.statement()?);
            }
            self.advance();
        } else {
            stmts.push(self.simple_statement()?);
        }

        Ok(stmts)
    }

    fn simple_statement(&mut self) -> Result<Stmt, CarlaeError> {
        if let Some(t) = self.peek() {
            // A variable assignment is a identifier followed by `=`
            let stmt = if matches!(t.kind, TokenKind::Identifier(_))
                && self.next_matches(&[TokenKind::Equal])
            {
                let name = t.clone();
                self.advance();
                self.advance();
                self.assignment_statement(name)?
            } else {
                let line = t.line;
                match &t.kind {
                    TokenKind::Print => {
                        self.advance();
                        self.print_stmt(line)?
                    }
                    _ => self.expression_stmt(line)?,
                }
            };
            Ok(stmt)
        } else {
            let prev = self.previous();
            Err(CarlaeError::Parsing(format!(
                "[Line {}] Expected token after {:?}",
                prev.line, prev.kind
            )))
        }
    }

    fn assignment_statement(&mut self, name: Token) -> Result<Stmt, CarlaeError> {
        let initializer = self.expression()?;

        if self.current_matches(&[TokenKind::Newline]) {
            self.advance();
            Ok(Stmt::Variable(Assignment { name, initializer }))
        } else {
            Err(CarlaeError::Parsing(format!(
                "[Line {}]: Invalid syntax for variable declaration",
                name.line
            )))
        }
    }

    fn print_stmt(&mut self, line: usize) -> Result<Stmt, CarlaeError> {
        let mut exprs: Vec<Expr> = Vec::new();
        let mut mode = PrintMode::FinalNewline;

        if !self.current_matches(&[TokenKind::Newline]) {
            exprs.push(self.expression()?);
        };
        while self.current_matches(&[TokenKind::Comma]) {
            self.advance();

            if self.current_matches(&[TokenKind::Newline]) {
                mode = PrintMode::NoNewline;
                break;
            } else {
                exprs.push(self.expression()?);
            }
        }
        if self.current_matches(&[TokenKind::Newline]) {
            self.advance();
            Ok(Stmt::Print(PrintConfig { exprs, mode }))
        } else {
            Err(CarlaeError::Parsing(format!(
                "[Line {line}] Invalid syntax for print statement",
            )))
        }
    }

    fn expression_stmt(&mut self, line: usize) -> Result<Stmt, CarlaeError> {
        let expr = self.expression()?;

        if self.current_matches(&[TokenKind::Newline]) {
            self.advance();
            Ok(Stmt::Expression(expr))
        } else {
            Err(CarlaeError::Parsing(format!(
                "[Line {line}] Missing newline after expression statement",
            )))
        }
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
                TokenKind::Identifier(_) => Expr::Variable(t.clone()),
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

    fn next_matches(&self, kinds: &[TokenKind]) -> bool {
        self.peek_next().is_some_and(|t| kinds.contains(&t.kind))
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

    fn peek_next(&self) -> Option<&Token> {
        self.tokens.get(self.current + 1)
    }

    fn previous(&self) -> &Token {
        self.current
            .checked_sub(1)
            .and_then(|i| self.tokens.get(i))
            .expect("Previous token exists")
    }

    fn synchronize(&mut self) -> Result<(), CarlaeError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_if_statement_with_inline_suite_no_else() {
        let mut parser = Parser::new(vec![
            Token::new(TokenKind::If, "if".into(), 1),
            Token::new(TokenKind::Number(1.0), "1".into(), 1),
            Token::new(TokenKind::Greater, ">".into(), 1),
            Token::new(TokenKind::Number(0.0), "0".into(), 1),
            Token::new(TokenKind::Colon, ":".into(), 1),
            Token::new(TokenKind::Identifier("x".into()), "x".into(), 1),
            Token::new(TokenKind::Equal, "=".into(), 1),
            Token::new(TokenKind::Number(1.0), "1".into(), 1),
            Token::new(TokenKind::Newline, "\n".into(), 1),
            Token::new(TokenKind::Eof, "".into(), 2),
        ]);
        let expected = Stmt::If(IfClause {
            cond: Expr::Binary {
                left: Box::new(Expr::Literal(LiteralValue::Number(1.0))),
                operator: Token::new(TokenKind::Greater, ">".into(), 1),
                right: Box::new(Expr::Literal(LiteralValue::Number(0.0))),
            },
            then: vec![Stmt::Variable(Assignment {
                name: Token::new(TokenKind::Identifier("x".into()), "x".into(), 1),
                initializer: Expr::Literal(LiteralValue::Number(1.0)),
            })],
            r#else: None,
        });

        let program = parser
            .parse()
            .expect("Tokens successfully parsed into statements");

        assert_eq!(program, vec![expected]);
    }

    #[test]
    fn parses_if_statement_with_indented_suite_and_else() {
        let mut parser = Parser::new(vec![
            Token::new(TokenKind::If, "if".into(), 1),
            Token::new(TokenKind::True, "True".into(), 1),
            Token::new(TokenKind::Colon, ":".into(), 1),
            Token::new(TokenKind::Newline, "\n".into(), 1),
            Token::new(TokenKind::Indent, "    ".into(), 2),
            Token::new(TokenKind::Identifier("x".into()), "x".into(), 2),
            Token::new(TokenKind::Equal, "=".into(), 2),
            Token::new(TokenKind::Number(1.0), "1".into(), 2),
            Token::new(TokenKind::Newline, "\n".into(), 2),
            Token::new(TokenKind::Dedent, "".into(), 3),
            Token::new(TokenKind::Else, "else".into(), 3),
            Token::new(TokenKind::Colon, ":".into(), 3),
            Token::new(TokenKind::Identifier("x".into()), "x".into(), 3),
            Token::new(TokenKind::Equal, "=".into(), 3),
            Token::new(TokenKind::Number(0.0), "0".into(), 3),
            Token::new(TokenKind::Newline, "\n".into(), 3),
            Token::new(TokenKind::Eof, "".into(), 4),
        ]);
        let expected = Stmt::If(IfClause {
            cond: Expr::Literal(LiteralValue::Boolean(true)),
            then: vec![Stmt::Variable(Assignment {
                name: Token::new(TokenKind::Identifier("x".into()), "x".into(), 2),
                initializer: Expr::Literal(LiteralValue::Number(1.0)),
            })],
            r#else: Some(vec![Stmt::Variable(Assignment {
                name: Token::new(TokenKind::Identifier("x".into()), "x".into(), 3),
                initializer: Expr::Literal(LiteralValue::Number(0.0)),
            })]),
        });

        let program = parser
            .parse()
            .expect("Tokens successfully parsed into statements");

        assert_eq!(program, vec![expected]);
    }

    #[test]
    fn parses_while_statement_with_two_line_body() {
        let mut parser = Parser::new(vec![
            Token::new(TokenKind::While, "while".into(), 1),
            Token::new(TokenKind::Identifier("x".into()), "x".into(), 1),
            Token::new(TokenKind::Colon, ":".into(), 1),
            Token::new(TokenKind::Newline, "\n".into(), 1),
            Token::new(TokenKind::Indent, "    ".into(), 2),
            Token::new(TokenKind::Identifier("x".into()), "x".into(), 2),
            Token::new(TokenKind::Equal, "=".into(), 2),
            Token::new(TokenKind::Number(0.0), "0".into(), 2),
            Token::new(TokenKind::Newline, "\n".into(), 2),
            Token::new(TokenKind::Print, "print".into(), 3),
            Token::new(TokenKind::Identifier("x".into()), "x".into(), 3),
            Token::new(TokenKind::Newline, "\n".into(), 3),
            Token::new(TokenKind::Dedent, "".into(), 4),
            Token::new(TokenKind::Eof, "".into(), 4),
        ]);
        let expected = Stmt::While(WhileClause {
            cond: Expr::Variable(Token::new(TokenKind::Identifier("x".into()), "x".into(), 1)),
            body: vec![
                Stmt::Variable(Assignment {
                    name: Token::new(TokenKind::Identifier("x".into()), "x".into(), 2),
                    initializer: Expr::Literal(LiteralValue::Number(0.0)),
                }),
                Stmt::Print(PrintConfig {
                    exprs: vec![Expr::Variable(Token::new(
                        TokenKind::Identifier("x".into()),
                        "x".into(),
                        3,
                    ))],
                    mode: PrintMode::FinalNewline,
                }),
            ],
        });

        let program = parser
            .parse()
            .expect("Tokens successfully parsed into statements");

        assert_eq!(program, vec![expected]);
    }

    #[test]
    fn parses_print_statement() {
        let mut parser = Parser::new(vec![
            Token::new(TokenKind::Print, "print".into(), 1),
            Token::new(TokenKind::Number(1.0), "1".into(), 1),
            Token::new(TokenKind::Plus, "+".into(), 1),
            Token::new(TokenKind::Number(2.0), "2".into(), 1),
            Token::new(TokenKind::Newline, "\n".into(), 1),
            Token::new(TokenKind::Eof, "".into(), 2),
        ]);
        let exprs = vec![Expr::Binary {
            left: Box::new(Expr::Literal(LiteralValue::Number(1.0))),
            operator: Token::new(TokenKind::Plus, "+".into(), 1),
            right: Box::new(Expr::Literal(LiteralValue::Number(2.0))),
        }];
        let expected = Stmt::Print(PrintConfig {
            exprs,
            mode: PrintMode::FinalNewline,
        });

        let program = parser
            .parse()
            .expect("Tokens successfully parsed into statements");

        assert_eq!(program, vec![expected]);
    }

    #[test]
    fn parses_print_statement_with_multiple_exprs() {
        let mut parser = Parser::new(vec![
            Token::new(TokenKind::Print, "print".into(), 1),
            Token::new(TokenKind::Number(1.0), "1".into(), 1),
            Token::new(TokenKind::Plus, "+".into(), 1),
            Token::new(TokenKind::Number(2.0), "2".into(), 1),
            Token::new(TokenKind::Comma, ",".into(), 1),
            Token::new(TokenKind::Number(3.0), "3".into(), 1),
            Token::new(TokenKind::Newline, "\n".into(), 1),
            Token::new(TokenKind::Eof, "".into(), 2),
        ]);
        let exprs = vec![
            Expr::Binary {
                left: Box::new(Expr::Literal(LiteralValue::Number(1.0))),
                operator: Token::new(TokenKind::Plus, "+".into(), 1),
                right: Box::new(Expr::Literal(LiteralValue::Number(2.0))),
            },
            Expr::Literal(LiteralValue::Number(3.0)),
        ];
        let expected = Stmt::Print(PrintConfig {
            exprs,
            mode: PrintMode::FinalNewline,
        });

        let program = parser
            .parse()
            .expect("Tokens successfully parsed into statements");

        assert_eq!(program, vec![expected]);
    }

    #[test]
    fn parses_print_statement_with_no_exprs() {
        let mut parser = Parser::new(vec![
            Token::new(TokenKind::Print, "print".into(), 1),
            Token::new(TokenKind::Newline, "\n".into(), 1),
            Token::new(TokenKind::Eof, "".into(), 2),
        ]);
        let expected = Stmt::Print(PrintConfig {
            exprs: Vec::new(),
            mode: PrintMode::FinalNewline,
        });

        let program = parser
            .parse()
            .expect("Tokens successfully parsed into statements");

        assert_eq!(program, vec![expected]);
    }

    #[test]
    fn parses_expression_statement() {
        let mut parser = Parser::new(vec![
            Token::new(TokenKind::Number(1.0), "1".into(), 1),
            Token::new(TokenKind::Plus, "+".into(), 1),
            Token::new(TokenKind::Number(2.0), "2".into(), 1),
            Token::new(TokenKind::Newline, "\n".into(), 1),
            Token::new(TokenKind::Eof, "".into(), 2),
        ]);
        let expected = Expr::Binary {
            left: Box::new(Expr::Literal(LiteralValue::Number(1.0))),
            operator: Token::new(TokenKind::Plus, "+".into(), 1),
            right: Box::new(Expr::Literal(LiteralValue::Number(2.0))),
        };

        let program = parser
            .parse()
            .expect("Tokens successfully parsed into statements");

        assert_eq!(program, vec![Stmt::Expression(expected)])
    }

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
            .expression()
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
            .expression()
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
            .expression()
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
            .expression()
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
            .expression()
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

        let result = parser.expression();

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

        let result = parser.expression();

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

        let result = parser.expression();

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
