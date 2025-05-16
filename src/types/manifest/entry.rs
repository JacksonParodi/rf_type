use crate::error::RfError;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use tracing::warn;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ManifestEntry {
    path: PathBuf,
    is_dir: bool,
    children: Option<Vec<ManifestEntry>>,
}
impl ManifestEntry {
    pub fn new(path: PathBuf, is_dir: bool) -> Self {
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
                path: path.clone(),
                is_dir: metadata.is_dir(),
                children,
            })
        }

        match build_manifest(&path) {
            Ok(manifest) => Ok(manifest),
            Err(e) => Err(RfError::IoError(e.to_string())),
        }
    }

    pub fn find_item(&self, file_name: &str) -> Option<PathBuf> {
        let mut result = None;
        if self.path.file_name()?.to_str()? == file_name {
            result = Some(self.path.clone());
        }
        if let Some(children) = &self.children {
            for child in children {
                if let Some(path) = child.find_item(file_name) {
                    result = Some(path);
                }
            }
        }

        match result {
            Some(path) => {
                Some(path)
                // let mut seen_asset = false;
                // let mut after_asset = Vec::new();

                // for comp in path.components() {
                //     if seen_asset {
                //         after_asset.push(comp.as_os_str());
                //     } else if comp
                //         .as_os_str()
                //         .to_string_lossy()
                //         .eq_ignore_ascii_case("asset")
                //     {
                //         seen_asset = true;
                //     }
                // }

                // if !seen_asset {
                //     warn!("Asset not found in path: {:?}", path);
                //     return None;
                // }

                // let relative_path = after_asset.iter().collect();
                // Some(relative_path)
            }
            None => None,
        }
    }
}
