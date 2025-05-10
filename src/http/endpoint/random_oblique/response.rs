use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::error;

#[derive(Serialize, Deserialize, Debug)]
pub struct RandomObliqueStratResponsePayload {
    pub oblique_strat: String,
}

impl RandomObliqueStratResponsePayload {
    pub fn new(oblique_strat: String) -> Self {
        RandomObliqueStratResponsePayload { oblique_strat }
    }
}

// continue with default, and From<Value> implementations

impl Default for RandomObliqueStratResponsePayload {
    fn default() -> Self {
        RandomObliqueStratResponsePayload {
            oblique_strat: String::new(),
        }
    }
}

impl From<Value> for RandomObliqueStratResponsePayload {
    fn from(value: Value) -> Self {
        match value {
            Value::String(oblique_strat) => RandomObliqueStratResponsePayload::new(oblique_strat),
            _ => {
                error!(
                    "Invalid value type for RandomObliqueStratResponsePayload: {:?}",
                    value
                );
                RandomObliqueStratResponsePayload::default()
            }
        }
    }
}
