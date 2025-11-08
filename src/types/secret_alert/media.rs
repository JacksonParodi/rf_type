use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Hash, Eq)]
pub struct NonvideoMedia {
    pub audio_filename: String,
    pub image_filename: String,
}

#[derive(Serialize, Deserialize, PartialEq, Hash, Eq)]
pub struct VideoMedia {
    pub video_filename: String,
}

#[derive(Serialize, Deserialize, PartialEq, Hash, Eq)]
/// the relevant filenames with no extensions
pub enum SecretAlertMedia {
    // video filename
    Video(VideoMedia),
    // (audio filename, image filename)
    Nonvideo(NonvideoMedia),
}
