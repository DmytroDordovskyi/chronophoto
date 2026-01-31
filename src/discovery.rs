use log::error;
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn discover_files(root: PathBuf) -> Vec<PathBuf> {
    WalkDir::new(root)
        .into_iter()
        .filter_map(|e| match e {
            Ok(entry) => Some(entry),
            Err(err) => {
                error!("Failed to access path during files discovery: {}", err);
                None
            }
        })
        .map(|e| e.into_path())
        .collect()
}
