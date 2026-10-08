use std::error::Error;

use crate::expr::LiteralValue;

#[derive(Debug)]
pub enum CarlaeError {
    Evaluation(String),
    General(String),
    Io(std::io::Error),
    Parsing(String),
    Return(Option<LiteralValue>),
    Scanning(String),
}

impl std::fmt::Display for CarlaeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Evaluation(s) => write!(f, "{s}"),
            Self::General(s) => write!(f, "{s}"),
            Self::Io(e) => write!(f, "{e}"),
            Self::Parsing(s) => write!(f, "{s}"),
            Self::Return(val) => {
                if let Some(v) = val {
                    write!(f, "return {v}")
                } else {
                    write!(f, "return")
                }
            }
            Self::Scanning(s) => write!(f, "{s}"),
        }
    }
}

impl Error for CarlaeError {}

impl From<std::io::Error> for CarlaeError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}
