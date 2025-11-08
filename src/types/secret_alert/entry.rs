use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq)]
pub struct SecretAlertEntry {
    /// the media filename with no extension
    pub filename: String,
    pub found: bool,
}
