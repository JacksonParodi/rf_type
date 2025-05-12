use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum IdentifyMessage {
    TwitchBot,
    DiscordBot,
    Overlay,
}
