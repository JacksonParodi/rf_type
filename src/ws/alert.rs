use serde::{Deserialize, Serialize};
use tracing::warn;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AlertData {
    pub id: String,
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
        let mut new_alert = Self {
            id: uuid::Uuid::new_v4().to_string(),
            text,
            image,
            sound,
            video,
        };

        if new_alert.text.is_none()
            && new_alert.image.is_none()
            && new_alert.sound.is_none()
            && new_alert.video.is_none()
        {
            warn!("AlertData created with all fields None");
        }

        if new_alert.video.is_some() && (new_alert.sound.is_some() || new_alert.image.is_some()) {
            warn!("AlertData created with video and other media types, removing other media types");
            new_alert.sound = None;
            new_alert.image = None;
        }

        new_alert
    }
}
