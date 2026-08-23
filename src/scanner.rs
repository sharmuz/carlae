use crate::error::CarlaeError;
use crate::token::{Token, TokenKind};

pub struct Scanner {
    pub source: String,
    pub tokens: Vec<Token>,
    pub start: usize,
    pub current: usize,
    pub line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    // TODO: Consider byte-based indexing instead of char-based
    pub fn scan_tokens(&mut self) -> Result<(), CarlaeError> {
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme
            self.start = self.current;
            self.scan_one_token()?;
        }

        self.tokens
            .push(Token::new(TokenKind::Eof, "".to_string(), self.line));
        Ok(())
    }

    fn scan_one_token(&mut self) -> Result<(), CarlaeError> {
        let ch = self.advance()?;

        match ch {
            '(' => self.add_token(TokenKind::LeftParen),
            ')' => self.add_token(TokenKind::RightParen),
            '+' => self.add_token(TokenKind::Plus),
            '-' => self.add_token(TokenKind::Minus),
            '*' => self.add_token(TokenKind::Star),
            '/' => self.add_token(TokenKind::Slash),
            '=' => {
                let kind = if self.matches_current('=') {
                    TokenKind::EqualEqual
                } else {
                    TokenKind::Equal
                };
                self.add_token(kind);
            }
            '!' => {
                let kind = if self.matches_current('=') {
                    TokenKind::BangEqual
                } else {
                    TokenKind::Bang
                };
                self.add_token(kind);
            }
            '>' => {
                let kind = if self.matches_current('=') {
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Greater
                };
                self.add_token(kind);
            }
            '<' => {
                let kind = if self.matches_current('=') {
                    TokenKind::LessEqual
                } else {
                    TokenKind::Less
                };
                self.add_token(kind);
            }
            n if n.is_ascii_digit() => self.number()?,
            '#' => {
                while let Some(ch) = self.peek() {
                    if ch != '\n' {
                        _ = self.advance()?;
                    } else {
                        break;
                    }
                }
            }
            x if x.is_whitespace() => {
                // TODO: Tokenize newline/indent/dedent whitespace
                if x == '\n' {
                    self.line += 1;
                }
            }
            _ => {
                return Err(CarlaeError::Scanning(format!(
                    "[Line {}] Unexpected character: {ch}",
                    self.line
                )));
            }
        }

        Ok(())
    }

    fn is_at_end(&self) -> bool {
        self.peek().is_none()
    }

    fn advance(&mut self) -> Result<char, CarlaeError> {
        let ch = self.peek().ok_or_else(|| CarlaeError::Scanning(format!(
            "[Line {}] Reached end of file unexpectedly",
            self.line
        )))?;
        self.current += 1;

        Ok(ch)
    }

    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.current)
    }

    fn peek_next(&self) -> Option<char> {
        self.source.chars().nth(self.current + 1)
    }

    fn matches_current(&mut self, expected: char) -> bool {
        if self.source.chars().nth(self.current) == Some(expected) {
            self.current += 1;
            return true;
        }
        false
    }

    fn number(&mut self) -> Result<(), CarlaeError> {
        while let Some(ch) = self.peek()
            && ch.is_ascii_digit()
        {
            _ = self.advance()?;
        }
        if let Some('.') = self.peek()
            && let Some(ch) = self.peek_next()
            && ch.is_ascii_digit()
        {
            _ = self.advance()?;

            while let Some(ch) = self.peek()
                && ch.is_ascii_digit()
            {
                _ = self.advance()?;
            }
        }
        let num = self.source_substring().collect::<String>();
        let float = num.parse::<f64>().map_err(|_| {
            CarlaeError::Scanning(format!(
                "[Line {}] Unable to parse to Number: {num}",
                self.line
            ))
        })?;
        self.add_token(TokenKind::Number(float));

        Ok(())
    }

    fn add_token(&mut self, kind: TokenKind) {
        let text: String = self.source_substring().collect();
        self.tokens.push(Token::new(kind, text, self.line))
    }

    fn source_substring(&self) -> impl Iterator<Item = char> {
        self.source.chars().take(self.current).skip(self.start)
    }
}
