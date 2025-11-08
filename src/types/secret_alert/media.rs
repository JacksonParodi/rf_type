use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Hash, Eq)]
/// the relevant filenames with no extensions
pub enum SecretAlertMedia {
    // video filename
    Video(String),
    // (audio filename, image filename)
    Nonvideo((String, String)),
}
