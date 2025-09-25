use crate::types::tts::TextToSpeechOptions;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::warn;

pub enum AlertMediaType {
    Video,
    Nonvideo,
}

impl AlertMediaType {
    pub fn random() -> Self {
        let mut rng = rand::rng();
        match rng.random_bool(0.5) {
            true => AlertMediaType::Video,
            false => AlertMediaType::Nonvideo,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AlertData {
    pub id: String,
    pub text: Option<String>,
    pub tts_options: Option<TextToSpeechOptions>,
    pub image: Option<PathBuf>,
    pub sound: Option<PathBuf>,
    pub video: Option<PathBuf>,
}

impl AlertData {
    pub fn new(
        text: Option<String>,
        tts: Option<TextToSpeechOptions>,
        image: Option<PathBuf>,
        sound: Option<PathBuf>,
        video: Option<PathBuf>,
    ) -> Self {
        let mut new_alert = Self {
            id: uuid::Uuid::new_v4().to_string(),
            tts_options: tts,
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

    pub fn dummy() -> Self {
        Self::new(Some("dummy alert text".to_string()), None, None, None, None)
    }
}
