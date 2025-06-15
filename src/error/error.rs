use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RfError {
    IoError(String),
    ParseError(String),
    NetworkError(String),
    SerenityError(String),
    DotenvyError(String),
    ReqwestError(String),
    TomlError(String),
    Other(String),
}

impl std::fmt::Display for RfError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RfError::IoError(msg) => write!(f, "IO Error: {}", msg),
            RfError::ParseError(msg) => write!(f, "Parse Error: {}", msg),
            RfError::NetworkError(msg) => write!(f, "Network Error: {}", msg),
            RfError::SerenityError(err) => write!(f, "Serenity Error: {}", err),
            RfError::DotenvyError(err) => write!(f, "Dotenvy Error: {}", err),
            RfError::ReqwestError(err) => write!(f, "Reqwest Error: {}", err),
            RfError::TomlError(err) => write!(f, "TOML Error: {}", err),
            RfError::Other(msg) => write!(f, "Other Error: {}", msg),
        }
    }
}

impl std::error::Error for RfError {}

impl From<std::io::Error> for RfError {
    fn from(err: std::io::Error) -> RfError {
        RfError::IoError(err.to_string())
    }
}

impl From<serenity::Error> for RfError {
    fn from(err: serenity::Error) -> RfError {
        RfError::SerenityError(err.to_string())
    }
}

impl From<dotenvy::Error> for RfError {
    fn from(err: dotenvy::Error) -> RfError {
        RfError::DotenvyError(err.to_string())
    }
}

impl From<reqwest::Error> for RfError {
    fn from(err: reqwest::Error) -> RfError {
        RfError::ReqwestError(err.to_string())
    }
}

impl From<toml::ser::Error> for RfError {
    fn from(err: toml::ser::Error) -> RfError {
        RfError::TomlError(err.to_string())
    }
}

impl From<toml::de::Error> for RfError {
    fn from(err: toml::de::Error) -> RfError {
        RfError::TomlError(err.to_string())
    }
}
