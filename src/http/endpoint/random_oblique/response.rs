use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RandomObliqueStratResponsePayload {
    pub oblique_strat: String,
}
