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
