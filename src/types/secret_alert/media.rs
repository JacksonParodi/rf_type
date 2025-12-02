use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Hash, Eq, Clone, Debug)]
pub struct NonvideoMedia {
    pub audio_filename: String,
    pub image_filename: String,
}

#[derive(Serialize, Deserialize, PartialEq, Hash, Eq, Clone, Debug)]
pub struct VideoMedia {
    pub video_filename: String,
}

#[derive(Serialize, Deserialize, PartialEq, Hash, Eq, Clone, Debug)]
/// the relevant filenames with no extensions
pub enum AlertMedia {
    // video filename
    Video(VideoMedia),
    // (audio filename, image filename)
    Nonvideo(NonvideoMedia),
}

impl Default for AlertMedia {
    fn default() -> Self {
        let mut rng = rand::rng();
        match rng.random_bool(0.5) {
            true => AlertMedia::Video(VideoMedia {
                video_filename: "default_video_alert".to_string(),
            }),
            false => AlertMedia::Nonvideo(NonvideoMedia {
                audio_filename: "default_audio_alert".to_string(),
                image_filename: "default_image_alert".to_string(),
            }),
        }
    }
}
