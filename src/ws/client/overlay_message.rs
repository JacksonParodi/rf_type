use crate::{
    types::donation::DonationData,
    ws::client::{ChatData, SubData},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum OverlayMessage {
    #[serde(rename = "chat")]
    Chat(ChatData),
    #[serde(rename = "sub")]
    Subscription(SubData),
    #[serde(rename = "donation")]
    Donation(DonationData),
}
