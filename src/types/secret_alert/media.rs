use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Hash, Eq, Clone)]
pub struct NonvideoMedia {
    pub audio_filename: String,
    pub image_filename: String,
}

#[derive(Serialize, Deserialize, PartialEq, Hash, Eq, Clone)]
pub struct VideoMedia {
    pub video_filename: String,
}

#[derive(Serialize, Deserialize, PartialEq, Hash, Eq, Clone)]
/// the relevant filenames with no extensions
pub enum AlertMedia {
    // video filename
    Video(VideoMedia),
    // (audio filename, image filename)
    Nonvideo(NonvideoMedia),
}

impl Default for AlertMedia {
    fn default() -> Self {
        AlertMedia::Nonvideo(NonvideoMedia {
            audio_filename: "default_alert".to_string(),
            image_filename: "default_alert".to_string(),
        })
    }
}
