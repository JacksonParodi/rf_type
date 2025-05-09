use crate::types::donation::Donation;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessNewDonationsResponsePayload {
    pub new_donations: HashMap<String, Donation>,
}
