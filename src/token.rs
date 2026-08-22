use std::str::FromStr;

use crate::error::CarlaeError;

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    lexeme: String,
    line: usize,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, line: usize) -> Self {
        Self { kind, lexeme, line }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.kind, self.lexeme)
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    // One-char tokens
    LeftParen,
    RightParen,
    Plus,
    Minus,
    Slash,
    Star,

    // Two-char tokens
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Number(f64),
    String(String),
    Identifier,

    // Keywords
    If,
    Else,
    True,
    False,
    None,

    // Other
    Newline,
    Indent,
    Dedent,
    Eof,
}

impl FromStr for TokenKind {
    type Err = CarlaeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // let num = s.parse::<f64>().map_err(|_| CarlaeError::Scan)?;

        if let Ok(num) = s.parse::<f64>() {
            Ok(Self::Number(num))
        } else {
            Err(CarlaeError::Scanning(format!(
                "Unable to parse to Number: {s}"
            )))
        }
    }
}
