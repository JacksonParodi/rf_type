use crate::error::RfError;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ManifestEntry {
    path: String,
    is_dir: bool,
    children: Option<Vec<ManifestEntry>>,
}
impl ManifestEntry {
    pub fn new(path: String, is_dir: bool) -> Self {
        ManifestEntry {
            path,
            is_dir,
            children: None,
        }
    }
    pub fn generate_recursive_manifest(path: &PathBuf) -> Result<Self, RfError> {
        fn build_manifest(path: &PathBuf) -> Result<ManifestEntry, RfError> {
            let metadata = fs::metadata(path).expect("failed to read metadata");
            let children: Option<Vec<ManifestEntry>> = if metadata.is_dir() {
                let mut entries = Vec::new();
                for entry in fs::read_dir(path).expect("failed to read directory") {
                    let entry = entry.expect("failed to read entry");
                    let child_path = entry.path();
                    entries.push(build_manifest(&child_path).expect("failed to build manifest"));
                }
                Some(entries)
            } else {
                None
            };
            Ok(ManifestEntry {
                path: path.to_string_lossy().to_string(),
                is_dir: metadata.is_dir(),
                children,
            })
        }

        match build_manifest(&path) {
            Ok(manifest) => Ok(manifest),
            Err(e) => Err(RfError::IoError(e.to_string())),
        }
    }
}
