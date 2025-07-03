use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DonationRequestAction {
    Fetch,
    Process,
    Add,
}

impl ToString for DonationRequestAction {
    fn to_string(&self) -> String {
        match self {
            DonationRequestAction::Fetch => "fetch".to_string(),
            DonationRequestAction::Process => "process".to_string(),
            DonationRequestAction::Add => "add".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DonationRequestFile {
    Old,
    New,
}

impl ToString for DonationRequestFile {
    fn to_string(&self) -> String {
        match self {
            DonationRequestFile::Old => "old".to_string(),
            DonationRequestFile::New => "new".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DonationRequestOptions {
    pub action: DonationRequestAction,
    pub file: Option<DonationRequestFile>,
    pub download: Option<bool>,
}
