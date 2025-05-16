use crate::ws::client::{AlertData, ChatData};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum OverlayMessage {
    #[serde(rename = "chat")]
    Chat(ChatData),
    #[serde(rename = "alert")]
    Alert(AlertData),
}
