use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ManifestEntry {
    path: String,
    is_dir: bool,
    children: Option<Vec<ManifestEntry>>,
}
