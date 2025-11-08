use crate::{
    error::RfError,
    types::secret_alert::{SecretAlertEntry, SecretAlertTrigger},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct SecretAlertManager {
    pub map: HashMap<SecretAlertTrigger, SecretAlertEntry>,
}

impl SecretAlertManager {
    pub fn new() -> Self {
        SecretAlertManager {
            map: HashMap::new(),
        }
    }

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
