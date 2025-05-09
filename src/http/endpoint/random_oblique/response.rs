use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GetRandomObliqueStratResponsePayload {
    pub oblique_strat: String,
    pub error: Option<String>,
}
