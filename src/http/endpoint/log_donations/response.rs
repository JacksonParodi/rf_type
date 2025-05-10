use crate::types::donation::DonationMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::error;

#[derive(Serialize, Deserialize, Debug)]
pub struct LogDonationsResponsePayload {
    pub new_donations: DonationMap,
}

impl LogDonationsResponsePayload {
    pub fn new(new_donations: DonationMap) -> Self {
        LogDonationsResponsePayload { new_donations }
    }
}

impl Default for LogDonationsResponsePayload {
    fn default() -> Self {
        LogDonationsResponsePayload {
            new_donations: DonationMap::default(),
        }
    }
}

impl From<Value> for LogDonationsResponsePayload {
    fn from(value: Value) -> Self {
        match value.clone() {
            Value::Object(_map) => LogDonationsResponsePayload::new(DonationMap::from(value)),
            _ => {
                error!("LogDonationsResponsePayload: Invalid value type: {}", value);
                LogDonationsResponsePayload::default()
            }
        }
    }
}
