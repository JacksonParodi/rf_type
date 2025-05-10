use crate::misc::constant;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::error;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FlintstoneResponsePayload {
    pub count: u64,
}

impl FlintstoneResponsePayload {
    pub fn new(count: u64) -> Self {
        FlintstoneResponsePayload { count }
    }
}

impl Default for FlintstoneResponsePayload {
    fn default() -> Self {
        FlintstoneResponsePayload {
            count: constant::FLINTSTONES_DEFAULT_COUNT,
        }
    }
}

impl From<Value> for FlintstoneResponsePayload {
    fn from(value: Value) -> Self {
        match value {
            Value::Number(num) => {
                if let Some(count) = num.as_u64() {
                    FlintstoneResponsePayload::new(count)
                } else {
                    error!("FlintstoneResponsePayload: Invalid number type");
                    FlintstoneResponsePayload::default()
                }
            }
            _ => {
                error!("FlintstoneResponsePayload: Invalid value type");
                FlintstoneResponsePayload::default()
            }
        }
    }
}
