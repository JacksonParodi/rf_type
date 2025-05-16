use serde::{Deserialize, Serialize};
use tracing::warn;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AlertData {
    pub text: Option<String>,
    pub image: Option<String>,
    pub sound: Option<String>,
    pub video: Option<String>,
}

impl AlertData {
    pub fn new(
        text: Option<String>,
        image: Option<String>,
        sound: Option<String>,
        video: Option<String>,
    ) -> Self {
        if text.is_none() && image.is_none() && sound.is_none() && video.is_none() {
            warn!("AlertData created with all fields None");
        }
        Self {
            text,
            image,
            sound,
            video,
        }
    }
}
