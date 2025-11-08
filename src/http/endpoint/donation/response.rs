use crate::types::donation::DonationMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::debug;

#[derive(Serialize, Deserialize, Debug)]
pub struct DonationResponsePayload {
    pub fetched_donations: Option<DonationMap>,
}

impl Default for DonationResponsePayload {
    fn default() -> Self {
        DonationResponsePayload {
            fetched_donations: None,
        }
    }
}

impl DonationResponsePayload {
    pub fn new(fetched_donations: DonationMap) -> Self {
        DonationResponsePayload {
            fetched_donations: Some(fetched_donations),
        }
    }
}

impl From<Value> for DonationResponsePayload {
    fn from(value: Value) -> Self {
        match value.clone() {
            Value::Object(_map) => DonationResponsePayload::new(DonationMap::from(value)),
            Value::Array(_array) => DonationResponsePayload::new(DonationMap::from(value)),
            _ => {
                debug!("DonationResponsePayload: Invalid value type: {}", value);
                DonationResponsePayload::default()
            }
        }
    }
}
