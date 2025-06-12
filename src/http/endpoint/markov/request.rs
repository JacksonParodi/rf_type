use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MarkovRequestParams {
    pub seed: Option<String>,
}
