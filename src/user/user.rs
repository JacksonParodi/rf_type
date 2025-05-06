use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: String,
    pub twitch_name: Option<String>,
    pub twitch_id: Option<String>,
    pub discord_id: Option<String>,
    pub discord_name: Option<String>,
    pub piasters: u64,
    pub inventory: Vec<String>,
}
