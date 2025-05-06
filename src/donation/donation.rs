use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Donation {
    pub id: String,
    pub donor_name: String,
    pub donor_message: String,
    pub amount: f64,
    pub currency: String,
    pub date: String,
}
