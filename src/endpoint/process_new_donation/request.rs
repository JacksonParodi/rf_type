use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessNewDonationRequestPayload {
    pub timestamp: u64,
}

impl ProcessNewDonationRequestPayload {
    pub fn new(timestamp: u64) -> Self {
        Self { timestamp }
    }
}
