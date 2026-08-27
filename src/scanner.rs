use std::collections::HashMap;
use std::sync::LazyLock;

use crate::error::CarlaeError;
use crate::token::{Token, TokenKind};

pub struct Scanner {
    pub source: String,
    pub tokens: Vec<Token>,
    pub start: usize,
    pub current: usize,
    pub line: usize,
    open_parens: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            open_parens: 0,
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
            '(' => {
                self.open_parens += 1;
                self.add_token(TokenKind::LeftParen);
            }
            ')' => {
                self.open_parens -= 1;
                self.add_token(TokenKind::RightParen);
            }
            '+' => self.add_token(TokenKind::Plus),
            '-' => self.add_token(TokenKind::Minus),
            '*' => self.add_token(TokenKind::Star),
            '/' => self.add_token(TokenKind::Slash),
            ',' => self.add_token(TokenKind::Comma),
            ':' => self.add_token(TokenKind::Colon),
            '=' => {
                let kind = if self.matches_current('=') {
                    TokenKind::EqualEqual
                } else {
                    TokenKind::Equal
                };
                self.add_token(kind);
            }
            '!' => {
                if self.matches_current('=') {
                    self.add_token(TokenKind::BangEqual);
                }
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
            '"' => self.string()?,
            s if s.is_alphanumeric() || s == '_' => self.identifier()?,
            '#' => {
                while let Some(ch) = self.peek()
                    && !matches!(ch, '\n' | '\r')
                {
                    self.advance()?;
                }
            }
            '\n' => {
                if self.open_parens == 0 {
                    self.end_logical_line();
                }
                self.line += 1;
            }
            '\r' => {
                if let Some('\n') = self.peek() {
                    self.advance()?;
                }
                if self.open_parens == 0 {
                    self.end_logical_line();
                }
                self.line += 1;
            }
            x if x.is_whitespace() => {
                // TODO: Tokenize indent/dedent whitespace
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
        let ch = self.peek().ok_or_else(|| {
            CarlaeError::Scanning(format!(
                "[Line {}] Reached end of file unexpectedly",
                self.line
            ))
        })?;
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
            self.advance()?;
        }
        if let Some('.') = self.peek()
            && let Some(ch) = self.peek_next()
            && ch.is_ascii_digit()
        {
            self.advance()?;

            while let Some(ch) = self.peek()
                && ch.is_ascii_digit()
            {
                self.advance()?;
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

    // TODO: Handle escaped strings
    fn string(&mut self) -> Result<(), CarlaeError> {
        while let Some(ch) = self.peek()
            && ch != '"'
        {
            self.advance()?;
        }
        self.advance()?;
        let mut str = self.source_substring().skip(1).collect::<String>();
        str.pop();
        self.add_token(TokenKind::String(str));

        Ok(())
    }

    fn identifier(&mut self) -> Result<(), CarlaeError> {
        while let Some(ch) = self.peek()
            && (ch.is_alphanumeric() || ch == '_')
        {
            self.advance()?;
        }
        let id = self.source_substring().collect();
        let kind = match KEYWORDS.get(&id) {
            Some(kind) => kind.clone(),
            None => TokenKind::Identifier(id),
        };
        self.add_token(kind);

        Ok(())
    }

    fn end_logical_line(&mut self) {
        let text: String = self.source_substring().collect();
        self.tokens
            .push(Token::new(TokenKind::Newline, text, self.line));
    }

    fn add_token(&mut self, kind: TokenKind) {
        let text: String = self.source_substring().collect();
        self.tokens.push(Token::new(kind, text, self.line))
    }

    fn source_substring(&self) -> impl Iterator<Item = char> {
        self.source.chars().take(self.current).skip(self.start)
    }
}

static KEYWORDS: LazyLock<HashMap<String, TokenKind>> = LazyLock::new(|| {
    HashMap::from([
        (String::from("if"), TokenKind::If),
        (String::from("else"), TokenKind::Else),
        (String::from("while"), TokenKind::While),
        (String::from("def"), TokenKind::Def),
        (String::from("return"), TokenKind::Return),
        (String::from("True"), TokenKind::True),
        (String::from("False"), TokenKind::False),
        (String::from("None"), TokenKind::None),
        (String::from("and"), TokenKind::And),
        (String::from("or"), TokenKind::Or),
        (String::from("not"), TokenKind::Not),
    ])
});
