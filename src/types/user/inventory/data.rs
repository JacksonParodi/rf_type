use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InventoryItemData {
    // Card(CardData),
    // Fish(FishData),
    // Animal(AnimalData),
    // Plant(PlantData),
    // Misc(MiscData),
    Dummy(u8),
}
