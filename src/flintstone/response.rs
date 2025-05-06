use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FlintstoneResponse {
    pub count: Option<u64>,
    pub error: Option<String>,
}
