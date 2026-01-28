use log::error;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

const ALLOWED_EXT: [&str; 3] = ["jpg", "jpeg", "png"];

pub fn discover_images(root: PathBuf) -> Vec<PathBuf> {
    WalkDir::new(root)
        .into_iter()
        .filter_map(|e| match e {
            Ok(entry) => Some(entry),
            Err(err) => {
                error!("Failed to access path during image discovery: {}", err);
                None
            }
        })
        .filter(has_allowed_extension)
        .map(|e| e.into_path())
        .collect()
}

fn has_allowed_extension(e: &DirEntry) -> bool {
    e.path()
        .extension()
        .and_then(|s| s.to_str())
        .map(|ext| ALLOWED_EXT.iter().any(|&e| e.eq_ignore_ascii_case(ext)))
        .unwrap_or(false)
}
