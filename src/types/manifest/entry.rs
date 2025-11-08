use crate::error::RfError;
use rand::Rng;
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

    pub fn find_item(&self, target_name: &str, is_dir: bool) -> Option<Self> {
        let mut result = None;

        if self.path.file_name()?.to_str()? == target_name {
            result = Some(self.to_owned());
        }

        let path = std::path::PathBuf::from(target_name);
        let path_no_ext = path.with_extension("");
        if self.path.file_stem()?.to_str()? == path_no_ext.file_stem()?.to_str()? {
            result = Some(self.to_owned());
        }

        if let Some(children) = &self.children {
            for child in children {
                if let Some(entry) = child.find_item(target_name, is_dir) {
                    if is_dir && !entry.is_dir {
                        continue;
                    }
                    if !is_dir && entry.is_dir {
                        continue;
                    }
                    result = Some(entry);
                }
            }
        }

        result
    }

    fn get_all_files_from_dir(&self, dir: &str) -> Vec<Self> {
        let mut files = Vec::new();

        if let Some(children) = &self.children {
            for child in children {
                if child.is_dir && child.path.file_name().unwrap().to_str().unwrap() == dir {
                    if let Some(child_files) = &child.children {
                        for file in child_files {
                            if !file.is_dir {
                                files.push(file.clone());
                            }
                        }
                    }
                } else if child.is_dir {
                    files.extend(child.get_all_files_from_dir(dir));
                }
            }
        }

        files
    }

    pub fn random_file_from_dir(&self, dir: &str) -> Option<Self> {
        let files = self.get_all_files_from_dir(dir);
        if files.is_empty() {
            None
        } else {
            let mut rng = rand::rng();
            let random_index = rng.random_range(0..files.len());
            Some(files[random_index].clone())
        }
    }

    pub fn find_file(&self, file_name: &str) -> Option<Self> {
        self.find_item(file_name, false)
    }

    pub fn find_dir(&self, dir_name: &str) -> Option<Self> {
        self.find_item(dir_name, true)
    }

    pub fn trimmed_path(&self) -> PathBuf {
        let mut rel_path = PathBuf::new();
        let mut seen_asset = false;

        for path_component in self.path.components() {
            if seen_asset {
                rel_path.push(path_component);
            }
            if path_component.as_os_str() == "asset" {
                seen_asset = true;
            }
        }

        if rel_path.components().count() == 0 {
            warn!("No asset directory found in path: {:?}", rel_path);
        }

        rel_path
    }
}
