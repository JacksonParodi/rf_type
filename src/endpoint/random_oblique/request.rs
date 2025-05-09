use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GetRandomObliqueStratRequestPayload {
    pub timestamp: u64,
}
