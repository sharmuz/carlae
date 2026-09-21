use std::collections::HashMap;

use crate::error::CarlaeError;
use crate::expr::LiteralValue;
use crate::token::Token;

pub struct Environment<'a> {
    values: HashMap<&'a String, LiteralValue>,
}

impl<'a> Environment<'a> {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn get(&self, name: &Token) -> Result<&LiteralValue, CarlaeError> {
        self.values
            .get(&(name.lexeme))
            .ok_or(CarlaeError::Evaluation(format!(
                "[Line {}]: Variable `{}` is not defined",
                name.line, name.lexeme
            )))
    }

    pub fn define(&mut self, name: &'a String, value: LiteralValue) -> Option<LiteralValue> {
        self.values.insert(name, value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::TokenKind;

    #[test]
    fn defines_and_gets_new_variable() {
        let mut env = Environment::new();
        let name = "foo".to_string();

        env.define(&name, LiteralValue::Number(2.0));
        let token = Token::new(
            TokenKind::Identifier("foo".to_string()),
            "foo".to_string(),
            1,
        );

        assert!(matches!(env.get(&token), Ok(&LiteralValue::Number(2.0))))
    }

    #[test]
    fn rejects_getting_non_existent_variable() {
        let mut env = Environment::new();
        let name = "foo".to_string();

        env.define(&name, LiteralValue::Number(2.0));
        let token = Token::new(
            TokenKind::Identifier("bar".to_string()),
            "bar".to_string(),
            1,
        );

        assert!(matches!(
            env.get(&token),
            Err(CarlaeError::Evaluation(msg))
            if msg.contains("not defined")
        ))
    }
}
