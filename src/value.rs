use std::rc::Rc;

use crate::error::CarlaeError;

#[derive(Clone, Debug)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    String(Rc<String>),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{n}"),
            Self::Boolean(true) => write!(f, "True"),
            Self::Boolean(false) => write!(f, "False"),
            Self::String(s) => write!(f, "\"{s}\""),
        }
    }
}

impl Value {
    pub fn as_number(&self) -> Result<f64, CarlaeError> {
        match self {
            Self::Number(n) => Ok(*n),
            Self::Boolean(b) => Ok((*b as i32) as f64),
            Self::String(_) => Err(CarlaeError::Runtime("Cannot convert str to float".into())),
        }
    }
}
