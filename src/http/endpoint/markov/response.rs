use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MarkovResponsePayload {
    pub id: String,
    pub text: String,
    pub error: Option<String>,
}
