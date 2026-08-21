use std::error::Error;

#[derive(Debug)]
pub enum CarlaeError {
    General(String),
    Io(std::io::Error),
    Scan,
}

impl std::fmt::Display for CarlaeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::General(s) => write!(f, "{s}"),
            Self::Io(e) => write!(f, "I/O error: {e}"),
            Self::Scan => write!(f, "Unable to scan source code!"),
        }
    }
}

impl Error for CarlaeError {}

impl From<std::io::Error> for CarlaeError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}
