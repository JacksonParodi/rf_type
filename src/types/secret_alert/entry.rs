use crate::types::secret_alert::SecretAlertTrigger;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq)]
pub struct SecretAlertEntry {
    pub filename: String,
    pub trigger: SecretAlertTrigger,
    pub found: bool,
}
