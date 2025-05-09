use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FlintstoneResponsePayload {
    pub count: u64,
}
