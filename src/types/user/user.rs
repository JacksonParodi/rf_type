use crate::types::user::inventory::InventoryItem;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: String,
    pub twitch_name: Option<String>,
    pub twitch_id: Option<String>,
    pub discord_id: Option<String>,
    pub discord_name: Option<String>,
    pub piasters: u64,
    pub inventory: HashMap<String, InventoryItem>,
}
