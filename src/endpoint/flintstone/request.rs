use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FlintstoneAction {
    GetCurrentCount,
    IncrementCount,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FlintstoneRequestPayload {
    pub action: FlintstoneAction,
}

impl FlintstoneRequestPayload {
    pub fn get_current_count() -> Self {
        FlintstoneRequestPayload {
            action: FlintstoneAction::GetCurrentCount,
        }
    }
    pub fn increment_count() -> Self {
        FlintstoneRequestPayload {
            action: FlintstoneAction::IncrementCount,
        }
    }
}
