use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::{error, warn};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MarkovResponsePayload {
    // successful text generation will have "Success" key
    pub text: String,
}

impl From<Value> for MarkovResponsePayload {
    fn from(value: Value) -> Self {
        match value {
            Value::Object(map) => {
                if let Some(text) = map.get("Success").and_then(|v| v.as_str()) {
                    MarkovResponsePayload {
                        text: text.to_string(),
                    }
                } else {
                    warn!("MarkovResponsePayload: 'Success' key not found in response");
                    MarkovResponsePayload {
                        text: "markov error...".to_string(),
                    }
                }
            }
            _ => {
                error!("MarkovResponsePayload: Invalid value type: {}", value);
                MarkovResponsePayload {
                    text: "markov error...".to_string(),
                }
            }
        }
    }
}
