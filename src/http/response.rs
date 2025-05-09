use crate::http::RfResponsePayload;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct HttpResponse {
    pub status: String,
    pub message: String,
    pub data: Option<RfResponsePayload>,
}
