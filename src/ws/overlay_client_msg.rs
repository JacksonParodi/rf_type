use crate::ws::{AlertData, ChatData};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum OverlayClientMessage {
    // this is a message that is intended to be handled directly by the overlay client in Javascript
    // it will be ferried by the overlay server
    #[serde(rename = "chat")]
    Chat(ChatData),
    #[serde(rename = "alert")]
    Alert(AlertData),
}
