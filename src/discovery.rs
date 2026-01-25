use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

const ALLOWED_EXT: [&str; 3] = ["jpg", "jpeg", "png"];

pub fn discover_images(root: PathBuf) -> Vec<PathBuf> {
    WalkDir::new(root)
        .into_iter()
        .filter_map(|e| match e {
            Ok(entry) => Some(entry),
            Err(err) => {
                eprintln!("Discovery images: {}", err);
                None
            }
        })
        .filter(with_allowed_ext)
        .map(|e| e.into_path())
        .collect()
}

fn with_allowed_ext(e: &DirEntry) -> bool {
    e.path()
        .extension()
        .and_then(|s| s.to_str())
        .map(|ext| ALLOWED_EXT.iter().any(|&e| e.eq_ignore_ascii_case(ext)))
        .unwrap_or(false)
}
