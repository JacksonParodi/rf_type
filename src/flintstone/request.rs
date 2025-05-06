use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FlintstoneAction {
    GetCurrentCount,
    IncrementCount,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FlintstoneRequest {
    pub action: FlintstoneAction,
}

impl FlintstoneRequest {
    pub fn get_current_count() -> Self {
        FlintstoneRequest {
            action: FlintstoneAction::GetCurrentCount,
        }
    }
    pub fn increment_count() -> Self {
        FlintstoneRequest {
            action: FlintstoneAction::IncrementCount,
        }
    }
}
