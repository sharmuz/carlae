#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: usize,
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

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    // One-char tokens
    LeftParen,
    RightParen,
    Plus,
    Minus,
    Slash,
    Star,
    Comma,
    Colon,

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
    Identifier(String),

    // Keywords
    If,
    Else,
    While,
    Def,
    Return,
    True,
    False,
    None,
    And,
    Or,
    Not,

    // Other
    Newline,
    Indent,
    Dedent,
    Eof,
}
