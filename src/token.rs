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
