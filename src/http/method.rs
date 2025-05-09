use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum HttpMethod {
    // so far, only GET and POST are used
    // but we can add more if needed
    GET,
    POST,
}
