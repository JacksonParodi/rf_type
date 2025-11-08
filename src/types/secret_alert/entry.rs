use crate::types::secret_alert::AlertMedia;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq)]
pub struct SecretAlertEntry {
    /// the media filename with no extension
    pub media: AlertMedia,
    pub found: bool,
}
