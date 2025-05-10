use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use tracing::error;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Donation {
    pub id: String,
    pub donor_name: String,
    pub donor_message: String,
    pub amount: f64,
    pub currency: String,
    pub date: String,
}

impl Donation {
    pub fn new(
        id: String,
        donor_name: String,
        donor_message: String,
        amount: f64,
        currency: String,
        date: String,
    ) -> Self {
        Donation {
            id,
            donor_name,
            donor_message,
            amount,
            currency,
            date,
        }
    }

    pub fn is_valid(&self) -> bool {
        !self.id.is_empty() && !self.donor_name.is_empty() && self.amount > 0.0
    }
}

impl Default for Donation {
    fn default() -> Self {
        Donation {
            id: String::new(),
            donor_name: String::new(),
            donor_message: String::new(),
            amount: 0.0,
            currency: String::new(),
            date: String::new(),
        }
    }
}

impl From<Value> for Donation {
    fn from(value: Value) -> Self {
        match value {
            Value::Object(map) => serde_json::from_value(Value::Object(map)).unwrap_or_default(),
            _ => {
                error!("Donation: Invalid value type: {}", value);
                Donation::default()
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DonationMap {
    pub donations: HashMap<String, Donation>,
}

impl DonationMap {
    pub fn new(donations: HashMap<String, Donation>) -> Self {
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
                    let donation: Donation = value.into();
                    match donation.is_valid() {
                        true => {
                            error!("DonationMap: Invalid donation: {:?}", donation);
                        }
                        false => {
                            hash_map.insert(key, donation);
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
