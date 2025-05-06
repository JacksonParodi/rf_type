use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MarkovResponse {
    pub id: String,
    pub text: String,
    pub error: Option<String>,
}
