use crate::types::secret_alert::SecretAlertMedia;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq)]
pub struct SecretAlertEntry {
    /// the media filename with no extension
    pub media: SecretAlertMedia,
    pub found: bool,
}
