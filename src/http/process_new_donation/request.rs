use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessNewDonationRequest {
    pub timestamp: u64,
}

impl ProcessNewDonationRequest {
    pub fn new(timestamp: u64) -> Self {
        Self { timestamp }
    }
}
