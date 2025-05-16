use crate::ws::role::RoleData;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OverlayServerMessage {
    // this is a message that is intended to be handled directly by the overlay server
    GetManifest,
    Role(RoleData),
}
