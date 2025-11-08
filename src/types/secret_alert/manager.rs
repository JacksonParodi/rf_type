use crate::{
    error::RfError,
    types::secret_alert::{SecretAlertEntry, SecretAlertTrigger},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct SecretAlertManager {
    #[serde(with = "trigger_map")]
    pub map: HashMap<SecretAlertTrigger, SecretAlertEntry>,
}

impl Default for SecretAlertManager {
    fn default() -> Self {
        SecretAlertManager {
            map: HashMap::new(),
        }
    }
}

impl SecretAlertManager {
    pub fn save_to_file(&self, file_path: &str) -> Result<(), RfError> {
        match toml::to_string_pretty(&self) {
            Ok(serialized) => match std::fs::write(file_path, serialized) {
                Ok(_) => Ok(()),
                Err(e) => {
                    return Err(RfError::IoError(format!(
                        "Failed to write SecretAlertManager to file: {}",
                        e
                    )));
                }
            },
            Err(e) => Err(RfError::IoError(format!(
                "Failed to serialize SecretAlertManager: {}",
                e
            ))),
        }
    }

    pub fn load_from_file(file_path: &str) -> Result<Self, RfError> {
        match std::fs::read_to_string(file_path) {
            Ok(content) => match toml::from_str::<SecretAlertManager>(&content) {
                Ok(manager) => Ok(manager),
                Err(e) => Err(RfError::IoError(format!(
                    "Failed to deserialize SecretAlertManager: {}",
                    e
                ))),
            },
            Err(e) => Err(RfError::IoError(format!(
                "Failed to read SecretAlertManager from file: {}",
                e
            ))),
        }
    }
}

mod trigger_map {
    use super::*;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(
        map: &HashMap<SecretAlertTrigger, SecretAlertEntry>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let string_map: HashMap<String, &SecretAlertEntry> =
            map.iter().map(|(k, v)| (trigger_to_string(k), v)).collect();
        string_map.serialize(serializer)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<HashMap<SecretAlertTrigger, SecretAlertEntry>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string_map: HashMap<String, SecretAlertEntry> = HashMap::deserialize(deserializer)?;
        let map = string_map
            .into_iter()
            .filter_map(|(k, v)| string_to_trigger(&k).map(|trigger| (trigger, v)))
            .collect();
        Ok(map)
    }

    fn trigger_to_string(trigger: &SecretAlertTrigger) -> String {
        match trigger {
            SecretAlertTrigger::Bits(amount) => format!("bits_{}", amount),
            SecretAlertTrigger::Donation(amount) => format!("donation_{}", amount),
            SecretAlertTrigger::GiftSubscription(count) => format!("gift_sub_{}", count),
            SecretAlertTrigger::Subscription(tier) => format!("subscription_{:?}", tier),
            SecretAlertTrigger::Raid => "raid".to_string(),
        }
    }

    fn string_to_trigger(s: &str) -> Option<SecretAlertTrigger> {
        if s == "raid" {
            return Some(SecretAlertTrigger::Raid);
        }

        if let Some(amount_str) = s.strip_prefix("bits_") {
            if let Ok(amount) = amount_str.parse::<u32>() {
                return Some(SecretAlertTrigger::Bits(amount));
            }
        }

        if let Some(amount_str) = s.strip_prefix("donation_") {
            if let Ok(amount) = amount_str.parse::<u64>() {
                return Some(SecretAlertTrigger::Donation(amount));
            }
        }

        if let Some(count_str) = s.strip_prefix("gift_sub_") {
            if let Ok(count) = count_str.parse::<u32>() {
                return Some(SecretAlertTrigger::GiftSubscription(count));
            }
        }

        if let Some(tier_str) = s.strip_prefix("subscription_") {
            match tier_str {
                "Prime" => Some(SecretAlertTrigger::Subscription(
                    crate::types::secret_alert::TwitchSubscriptionTier::Prime,
                )),
                "Tier1" => Some(SecretAlertTrigger::Subscription(
                    crate::types::secret_alert::TwitchSubscriptionTier::Tier1,
                )),
                "Tier2" => Some(SecretAlertTrigger::Subscription(
                    crate::types::secret_alert::TwitchSubscriptionTier::Tier2,
                )),
                "Tier3" => Some(SecretAlertTrigger::Subscription(
                    crate::types::secret_alert::TwitchSubscriptionTier::Tier3,
                )),
                _ => None,
            }
        } else {
            None
        }
    }
}
