use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FlintstoneResponsePayload {
    pub count: Option<u64>,
    pub error: Option<String>,
}
