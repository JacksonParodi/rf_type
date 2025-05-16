use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ChatColor {
    Red,
    Green,
    Blue,
    Yellow,
    Purple,
    Orange,
    Pink,
    White,
}

impl ChatColor {
    pub fn random() -> Self {
        let mut rng = rand::rng();
        let color_index = rng.random_range(0..8);
        match color_index {
            0 => ChatColor::Red,
            1 => ChatColor::Green,
            2 => ChatColor::Blue,
            3 => ChatColor::Yellow,
            4 => ChatColor::Purple,
            5 => ChatColor::Orange,
            6 => ChatColor::Pink,
            _ => ChatColor::White,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatData {
    pub name: String,
    pub text: String,
    pub color: ChatColor,
}

impl ChatData {
    pub fn new(name: String, text: String, color: ChatColor) -> Self {
        Self { name, text, color }
    }
}
