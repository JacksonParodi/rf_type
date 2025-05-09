use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MarkovRequestPayload {
    pub seed: Option<String>,
    pub max_words: Option<u32>,
    pub min_words: Option<u32>,
}
