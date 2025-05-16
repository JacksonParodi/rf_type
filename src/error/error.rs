use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RfError {
    IoError(String),
    ParseError(String),
    NetworkError(String),
    Other(String),
}

impl std::fmt::Display for RfError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RfError::IoError(msg) => write!(f, "IO Error: {}", msg),
            RfError::ParseError(msg) => write!(f, "Parse Error: {}", msg),
            RfError::NetworkError(msg) => write!(f, "Network Error: {}", msg),
            RfError::Other(msg) => write!(f, "Other Error: {}", msg),
        }
    }
}

impl std::error::Error for RfError {}
