use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubData {
    pub name: String,
    pub months: u32,
    pub message: Option<String>,
}
