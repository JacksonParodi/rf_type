use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use tracing::error;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DonationData {
    pub id: String,
    pub donor_name: String,
    pub donor_message: String,
    pub amount: u64,
    pub currency: String,
    pub date: String,
}

impl DonationData {
    pub fn new(
        id: String,
        donor_name: String,
        donor_message: String,
        amount: u64,
        currency: String,
        date: String,
    ) -> Self {
        DonationData {
            id,
            donor_name,
            donor_message,
            amount,
            currency,
            date,
        }
    }

    pub fn is_valid(&self) -> bool {
        !self.id.is_empty() && !self.donor_name.is_empty() && self.amount > 0
    }

    pub fn to_display_string(&self) -> String {
        let amount_string = format!("{:.2}", self.amount as f64 / 100.0);
        format!(
            "{} tipped {} {}. {}",
            self.donor_name, amount_string, self.currency, self.donor_message
        )
    }
}

impl Default for DonationData {
    fn default() -> Self {
        DonationData {
            id: String::new(),
            donor_name: String::new(),
            donor_message: String::new(),
            amount: 0,
            currency: String::new(),
            date: String::new(),
        }
    }
}

impl From<Value> for DonationData {
    fn from(value: Value) -> Self {
        match value {
            Value::Object(map) => {
                let id = map
                    .get("id")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_string();

                let donor_name = map
                    .get("name")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_string();

                let donor_message = map
                    .get("message")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_string();

                let amount_float = map
                    .get("amount")
                    .and_then(Value::as_str)
                    .and_then(|s| s.parse::<f64>().ok())
                    .unwrap_or(0.0);

                // convert amount to cents (u64)
                // how does it work for non-USD currencies? no clue, would be a nice problem to have
                let amount = (amount_float * 100.0).round() as u64;

                let currency = map
                    .get("currency")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_string();

                let timestamp = map
                    .get("timestamp")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_string();

                DonationData::new(id, donor_name, donor_message, amount, currency, timestamp)
            }
            _ => {
                error!("Donation: Invalid value type: {}", value);
                DonationData::default()
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DonationMap {
    pub donations: HashMap<String, DonationData>,
}

impl DonationMap {
    pub fn new(donations: HashMap<String, DonationData>) -> Self {
        DonationMap { donations }
    }
}

impl Default for DonationMap {
    fn default() -> Self {
        DonationMap {
            donations: HashMap::new(),
        }
    }
}

impl From<Value> for DonationMap {
    fn from(value: Value) -> Self {
        match value {
            Value::Object(map) => {
                let mut hash_map = HashMap::new();

                for (key, value) in map {
                    let donation = DonationData::from(value);
                    match donation.is_valid() {
                        true => {
                            hash_map.insert(key, donation);
                        }
                        false => {
                            error!("DonationMap: Invalid donation: {:?}", donation);
                        }
                    }
                }
                DonationMap::new(hash_map)
            }
            Value::Array(array) => {
                let mut hash_map = HashMap::new();

                for value in array {
                    let donation = DonationData::from(value);
                    match donation.is_valid() {
                        true => {
                            hash_map.insert(donation.id.clone(), donation);
                        }
                        false => {
                            error!("DonationMap: Invalid donation: {:?}", donation);
                        }
                    }
                }
                DonationMap::new(hash_map)
            }
            _ => {
                error!("DonationMap: Invalid value type: {}", value);
                DonationMap::default()
            }
        }
    }
}
