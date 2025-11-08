use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DonationRequestAction {
    Fetch,
    Add,
}

impl ToString for DonationRequestAction {
    fn to_string(&self) -> String {
        match self {
            DonationRequestAction::Fetch => "fetch".to_string(),
            DonationRequestAction::Add => "add".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DonationRequestOptions {
    pub action: DonationRequestAction,
    pub amount: Option<u8>,
}
