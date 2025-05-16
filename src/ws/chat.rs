use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatData {
    pub name: String,
    pub text: String,
}

impl ChatData {
    pub fn new(name: String, text: String) -> Self {
        Self { name, text }
    }
}
