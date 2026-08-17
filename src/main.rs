use std::{error::Error, str::FromStr};

fn main() -> Result<(), Box<dyn Error>> {
    let input = "123 32.0 87\t12.3  0\n";
    println!("\nInput is: {input}");
    let tokens = scan(&input)?;
    println!("\nTokens are: {tokens:?}");
    Ok(())
}

#[derive(Debug, PartialEq)]
enum Token {
    Number(Number),
}

impl FromStr for Token {
    type Err = ScanError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::Number(Number::from_str(s)?))
    }
}

#[derive(Debug, PartialEq)]
enum Number {
    Integer(isize),
    Float(f64),
}

impl FromStr for Number {
    type Err = ScanError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(val) = s.parse::<isize>() {
            Ok(Self::Integer(val))
        } else {
            let val = s.parse::<f64>().map_err(|_| ScanError)?;
            Ok(Self::Float(val))
        }
    }
}

// NEWLINE
// INDENT
// DEDENT

// identifiers + keywords
// NAME

// Literals
// NUMBER
// STRING
// BYTES

// Operators + delimeters
// OP
// LPAR
// RPAR
// PLUS
// MINUS
// STAR
// SLASH
// EQUAL

#[derive(Debug)]
struct ScanError;

impl std::fmt::Display for ScanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unable to scan!")
    }
}

impl Error for ScanError {}

fn scan(input: &str) -> Result<Vec<Token>, ScanError> {
    let mut tokens = vec![];
    let mut t = String::new();

    for ch in input.chars() {
        if ch.is_whitespace() {
            if !t.is_empty() {
                tokens.push(Token::from_str(&t)?);
                t.clear();
            }
        } else {
            t.push(ch);
        }
    }

    if !t.is_empty() {
        tokens.push(Token::from_str(&t)?);
    }

    Ok(tokens)
}

// fn parse

// fn eval

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_whitespace_delim_numbers() {
        let input = "123 32.0 87\t12.3  0\n";
        let expected: Vec<Token> = vec![
            Token::Number(Number::Integer(123)),
            Token::Number(Number::Float(32.0)),
            Token::Number(Number::Integer(87)),
            Token::Number(Number::Float(12.3)),
            Token::Number(Number::Integer(0)),
        ];

        let tokens = scan(&input).expect("Input string scanned into tokens");

        assert_eq!(tokens, expected);
    }
}
