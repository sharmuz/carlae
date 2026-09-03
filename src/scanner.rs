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
    indent_stack: Vec<usize>,
    indent_char: Option<char>,
    blank_line: bool,
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
            indent_stack: vec![0],
            indent_char: None,
            blank_line: true,
        }
    }

    // TODO: Consider byte-based indexing instead of char-based
    pub fn scan_tokens(&mut self) -> Result<(), CarlaeError> {
        // Note: Parser will handle initial (invalid) indent if produced
        self.indent_dedent()?;

        while !self.is_at_end() {
            // We are at the beginning of the next lexeme
            self.start = self.current;
            self.scan_one_token()?;
        }

        self.end_of_input();

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
                self.open_parens = self.open_parens.saturating_sub(1);
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
            '\n' => self.end_physical_line(1)?,
            '\r' => {
                let mut start_offset = 1;
                if let Some('\n') = self.peek() {
                    self.advance()?;
                    start_offset += 1;
                }
                self.end_physical_line(start_offset)?
            }
            '\\' => {
                match (self.peek(), self.peek_next()) {
                    (Some('\r'), Some('\n')) => {
                        self.advance()?;
                        self.advance()?;
                    }
                    (Some('\n' | '\r'), _) => {
                        self.advance()?;
                    }
                    _ => {
                        return Err(CarlaeError::Scanning(format!(
                            "[Line {}] Unexpected character: {ch}",
                            self.line
                        )));
                    }
                }
                self.line += 1;
            }
            x if x.is_whitespace() => {
                // Remaining whitespace can be skipped
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
        self.add_token(TokenKind::Newline);
        self.blank_line = true;
    }

    fn end_physical_line(&mut self, start_offset: usize) -> Result<(), CarlaeError> {
        if self.open_parens == 0 {
            if !self.blank_line {
                self.end_logical_line();
            }
            self.line += 1;
            self.start += start_offset;
            self.indent_dedent()?;
        } else {
            self.line += 1;
        }

        Ok(())
    }

    fn indent_dedent(&mut self) -> Result<(), CarlaeError> {
        let Some(indent_level) = self.calc_indent_level()? else {
            return Ok(());
        };

        // No indent/dedent if EOF
        if self.is_at_end() {
            return Ok(());
        }

        let mut last_indent = self
            .indent_stack
            .last()
            .expect("Indent stack should always be non-empty");
        if indent_level > *last_indent {
            self.indent_stack.push(indent_level);
            let text: String = self.source_substring().collect();
            self.tokens
                .push(Token::new(TokenKind::Indent, text, self.line))
        } else if indent_level < *last_indent {
            if !self.indent_stack.contains(&indent_level) {
                return Err(CarlaeError::Scanning(format!(
                    "[Line {}] Unexpected indentation level",
                    self.line
                )));
            }
            while indent_level < *last_indent {
                self.indent_stack.pop();
                self.tokens
                    .push(Token::new(TokenKind::Dedent, "".to_string(), self.line));
                last_indent = self
                    .indent_stack
                    .last()
                    .expect("Indent stack should always be non-empty");
            }
        }

        Ok(())
    }

    fn calc_indent_level(&mut self) -> Result<Option<usize>, CarlaeError> {
        let mut indent_level: usize = 0;
        if let Some('\x0C') = self.peek() {
            self.advance()?;
        };
        if self.indent_char.is_none() {
            self.indent_char = if let Some(' ' | '\t') = self.peek() {
                self.peek()
            } else {
                None
            };
        }
        while let Some(ch) = self.peek()
            && let Some(id) = self.indent_char
        {
            match (ch, id) {
                (' ', ' ') => {
                    self.advance()?;
                    indent_level += 1;
                }
                ('\t', '\t') => {
                    self.advance()?;
                    indent_level += 8;
                }
                (x @ (' ' | '\t'), y) if x != y => {
                    return Err(CarlaeError::Scanning(format!(
                        "[Line {}] Input uses mixture of spaces and tabs for indentation",
                        self.line
                    )));
                }
                ('\x0C', _) => {
                    return Err(CarlaeError::Scanning(format!(
                        "[Line {}] Formfeed char only permitted at start of line",
                        self.line
                    )));
                }
                // No indent/dedent if line ends without non-whitespace/comment char
                ('\n' | '\r' | '#', _) => return Ok(None),
                _ => break,
            }
        }

        Ok(Some(indent_level))
    }

    fn end_of_input(&mut self) {
        if !self.blank_line {
            self.tokens
                .push(Token::new(TokenKind::Newline, "\n".to_string(), self.line));
        }
        while self
            .indent_stack
            .pop()
            .expect("Indent stack should always be non-empty")
            > 0
        {
            self.tokens
                .push(Token::new(TokenKind::Dedent, "".to_string(), self.line));
        }
        self.tokens
            .push(Token::new(TokenKind::Eof, "".to_string(), self.line));
    }

    fn add_token(&mut self, kind: TokenKind) {
        self.blank_line = false;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scans_basic_op_in_parens() {
        let source = "(1 + 2)\n".to_string();
        let mut scanner = Scanner::new(source);
        let expected = vec![
            Token::new(TokenKind::LeftParen, "(".into(), 1),
            Token::new(TokenKind::Number(1.0), "1".into(), 1),
            Token::new(TokenKind::Plus, "+".into(), 1),
            Token::new(TokenKind::Number(2.0), "2".into(), 1),
            Token::new(TokenKind::RightParen, ")".into(), 1),
            Token::new(TokenKind::Newline, "\n".into(), 1),
            Token::new(TokenKind::Eof, "".into(), 2),
        ];

        scanner.scan_tokens().expect("Tokens generated from source");

        assert_eq!(scanner.tokens, expected);
    }

    #[test]
    fn scans_basic_string_assignment() {
        let source = "drink = \"banana_smoothie\"\n".to_string();
        let mut scanner = Scanner::new(source);
        let expected = vec![
            Token::new(TokenKind::Identifier("drink".into()), "drink".into(), 1),
            Token::new(TokenKind::Equal, "=".into(), 1),
            Token::new(
                TokenKind::String("banana_smoothie".into()),
                "\"banana_smoothie\"".into(),
                1,
            ),
            Token::new(TokenKind::Newline, "\n".into(), 1),
            Token::new(TokenKind::Eof, "".into(), 2),
        ];

        scanner.scan_tokens().expect("Tokens generated from source");

        assert_eq!(scanner.tokens, expected);
    }

    #[test]
    fn scans_single_line_if_else() {
        let source = r#"x = "apple" if y == "yes" else "orange""#.to_string();
        let mut scanner = Scanner::new(source);
        let expected = vec![
            Token::new(TokenKind::Identifier("x".into()), "x".into(), 1),
            Token::new(TokenKind::Equal, "=".into(), 1),
            Token::new(TokenKind::String("apple".into()), "\"apple\"".into(), 1),
            Token::new(TokenKind::If, "if".into(), 1),
            Token::new(TokenKind::Identifier("y".into()), "y".into(), 1),
            Token::new(TokenKind::EqualEqual, "==".into(), 1),
            Token::new(TokenKind::String("yes".into()), "\"yes\"".into(), 1),
            Token::new(TokenKind::Else, "else".into(), 1),
            Token::new(TokenKind::String("orange".into()), "\"orange\"".into(), 1),
            Token::new(TokenKind::Newline, "\n".into(), 1),
            Token::new(TokenKind::Eof, "".into(), 1),
        ];

        scanner.scan_tokens().expect("Tokens generated from source");

        assert_eq!(scanner.tokens, expected);
    }

    #[test]
    fn scans_lines_with_cr_lf() {
        let source = "x = 1 + 2\ny = 2 + 3\rz = x * y\r\n".to_string();
        let mut scanner = Scanner::new(source);
        let expected = vec![
            Token::new(TokenKind::Identifier("x".into()), "x".into(), 1),
            Token::new(TokenKind::Equal, "=".into(), 1),
            Token::new(TokenKind::Number(1.0), "1".into(), 1),
            Token::new(TokenKind::Plus, "+".into(), 1),
            Token::new(TokenKind::Number(2.0), "2".into(), 1),
            Token::new(TokenKind::Newline, "\n".into(), 1),
            Token::new(TokenKind::Identifier("y".into()), "y".into(), 2),
            Token::new(TokenKind::Equal, "=".into(), 2),
            Token::new(TokenKind::Number(2.0), "2".into(), 2),
            Token::new(TokenKind::Plus, "+".into(), 2),
            Token::new(TokenKind::Number(3.0), "3".into(), 2),
            Token::new(TokenKind::Newline, "\r".into(), 2),
            Token::new(TokenKind::Identifier("z".into()), "z".into(), 3),
            Token::new(TokenKind::Equal, "=".into(), 3),
            Token::new(TokenKind::Identifier("x".into()), "x".into(), 3),
            Token::new(TokenKind::Star, "*".into(), 3),
            Token::new(TokenKind::Identifier("y".into()), "y".into(), 3),
            Token::new(TokenKind::Newline, "\r\n".into(), 3),
            Token::new(TokenKind::Eof, "".into(), 4),
        ];

        scanner.scan_tokens().expect("Tokens generated from source");

        assert_eq!(scanner.tokens, expected);
    }

    #[test]
    fn scans_lines_with_implicit_continuation() {
        let source = "x = (\n1 + (\\\n2 + 3\n)\n)\ny = 4".to_string();
        let mut scanner = Scanner::new(source);
        let expected = vec![
            Token::new(TokenKind::Identifier("x".into()), "x".into(), 1),
            Token::new(TokenKind::Equal, "=".into(), 1),
            Token::new(TokenKind::LeftParen, "(".into(), 1),
            Token::new(TokenKind::Number(1.0), "1".into(), 2),
            Token::new(TokenKind::Plus, "+".into(), 2),
            Token::new(TokenKind::LeftParen, "(".into(), 2),
            Token::new(TokenKind::Number(2.0), "2".into(), 3),
            Token::new(TokenKind::Plus, "+".into(), 3),
            Token::new(TokenKind::Number(3.0), "3".into(), 3),
            Token::new(TokenKind::RightParen, ")".into(), 4),
            Token::new(TokenKind::RightParen, ")".into(), 5),
            Token::new(TokenKind::Newline, "\n".into(), 5),
            Token::new(TokenKind::Identifier("y".into()), "y".into(), 6),
            Token::new(TokenKind::Equal, "=".into(), 6),
            Token::new(TokenKind::Number(4.0), "4".into(), 6),
            Token::new(TokenKind::Newline, "\n".into(), 6),
            Token::new(TokenKind::Eof, "".into(), 6),
        ];

        scanner.scan_tokens().expect("Tokens generated from source");

        assert_eq!(scanner.tokens, expected);
    }

    #[test]
    fn scans_lines_with_indent_dedent() {
        let source = "def func():\n    if True:\n        1\n    else:\n        0".to_string();
        let mut scanner = Scanner::new(source);
        let expected = vec![
            Token::new(TokenKind::Def, "def".into(), 1),
            Token::new(TokenKind::Identifier("func".into()), "func".into(), 1),
            Token::new(TokenKind::LeftParen, "(".into(), 1),
            Token::new(TokenKind::RightParen, ")".into(), 1),
            Token::new(TokenKind::Colon, ":".into(), 1),
            Token::new(TokenKind::Newline, "\n".into(), 1),
            Token::new(TokenKind::Indent, "    ".into(), 2),
            Token::new(TokenKind::If, "if".into(), 2),
            Token::new(TokenKind::True, "True".into(), 2),
            Token::new(TokenKind::Colon, ":".into(), 2),
            Token::new(TokenKind::Newline, "\n".into(), 2),
            Token::new(TokenKind::Indent, "        ".into(), 3),
            Token::new(TokenKind::Number(1.0), "1".into(), 3),
            Token::new(TokenKind::Newline, "\n".into(), 3),
            Token::new(TokenKind::Dedent, "".into(), 4),
            Token::new(TokenKind::Else, "else".into(), 4),
            Token::new(TokenKind::Colon, ":".into(), 4),
            Token::new(TokenKind::Newline, "\n".into(), 4),
            Token::new(TokenKind::Indent, "        ".into(), 5),
            Token::new(TokenKind::Number(0.0), "0".into(), 5),
            Token::new(TokenKind::Newline, "\n".into(), 5),
            Token::new(TokenKind::Dedent, "".into(), 5),
            Token::new(TokenKind::Dedent, "".into(), 5),
            Token::new(TokenKind::Eof, "".into(), 5),
        ];

        scanner.scan_tokens().expect("Tokens generated from source");

        assert_eq!(scanner.tokens, expected);
    }

    #[test]
    fn rejects_unexpected_character() {
        let source = "1\n.\n2".to_string();
        let mut scanner = Scanner::new(source);
        let expected = "line 2";

        let result = scanner.scan_tokens();

        assert!(matches!(
            result,
            Err(CarlaeError::Scanning(message))
                if message.to_lowercase().contains(expected)
        ));
    }

    #[test]
    fn rejects_unterminated_string() {
        let source = "1\n\"text".to_string();
        let mut scanner = Scanner::new(source);
        let expected = "line 2";

        let result = scanner.scan_tokens();

        assert!(matches!(
            result,
            Err(CarlaeError::Scanning(message))
                if message.to_lowercase().contains(expected)
        ));
    }

    #[test]
    fn rejects_invalid_dedent() {
        let source = "1\n  1\n 1\n2".to_string();
        let mut scanner = Scanner::new(source);
        let expected = "line 3";

        let result = scanner.scan_tokens();

        assert!(matches!(
            result,
            Err(CarlaeError::Scanning(message))
                if message.to_lowercase().contains(expected)
        ));
    }

    #[test]
    fn skips_blank_and_comment_only_lines() {
        let source = "1\n\n# comment\n2".to_string();
        let mut scanner = Scanner::new(source);
        let expected = vec![
            Token::new(TokenKind::Number(1.0), "1".into(), 1),
            Token::new(TokenKind::Newline, "\n".into(), 1),
            Token::new(TokenKind::Number(2.0), "2".into(), 4),
            Token::new(TokenKind::Newline, "\n".into(), 4),
            Token::new(TokenKind::Eof, "".into(), 4),
        ];

        scanner.scan_tokens().expect("Tokens generated from source");

        assert_eq!(scanner.tokens, expected);
    }
}
