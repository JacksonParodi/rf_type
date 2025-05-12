use crate::misc::constant;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Role {
    TwitchBot,
    DiscordBot,
    Overlay,
}

impl Role {
    pub fn as_bytes(&self) -> [u8; 5] {
        let mut result_bytes = [0; 5];
        result_bytes[0..3].copy_from_slice(&constant::ROLE_HEADER_BYTES);

        let role_bytes = match self {
            Role::TwitchBot => constant::ROLE_TWITCH_BOT_BYTES,
            Role::DiscordBot => constant::ROLE_DISCORD_BOT_BYTES,
            Role::Overlay => constant::ROLE_OVERLAY_BYTES,
        };
        result_bytes[3..5].copy_from_slice(&role_bytes);

        result_bytes
    }
}
