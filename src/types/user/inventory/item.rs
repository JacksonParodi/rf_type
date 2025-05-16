use crate::types::user::inventory::data::InventoryItemData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryItem {
    pub id: String,
    pub data: InventoryItemData,
}
