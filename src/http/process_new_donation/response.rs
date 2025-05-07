use crate::donation::Donation;
use std::collections::HashMap;

pub struct ProcessNewDonationResponse {
    pub new_donations: HashMap<String, Donation>,
}
