use crate::types::donation::DonationMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::error;

#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessNewDonationsResponsePayload {
    pub new_donations: DonationMap,
}

impl ProcessNewDonationsResponsePayload {
    pub fn new(new_donations: DonationMap) -> Self {
        ProcessNewDonationsResponsePayload { new_donations }
    }
}

impl Default for ProcessNewDonationsResponsePayload {
    fn default() -> Self {
        ProcessNewDonationsResponsePayload {
            new_donations: DonationMap::default(),
        }
    }
}

impl From<Value> for ProcessNewDonationsResponsePayload {
    fn from(value: Value) -> Self {
        match value {
            Value::Object(map) => {
                let new_donations: DonationMap =
                    serde_json::from_value(Value::Object(map)).unwrap_or_default();
                ProcessNewDonationsResponsePayload::new(new_donations)
            }
            _ => {
                error!(
                    "ProcessNewDonationsResponsePayload: Invalid value type: {}",
                    value
                );
                ProcessNewDonationsResponsePayload::default()
            }
        }
    }
}
