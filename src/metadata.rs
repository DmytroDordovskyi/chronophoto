use crate::exif_extractor::extract;
use crate::types::PhotoMetadata;
use std::path::PathBuf;

pub fn convert(paths: Vec<PathBuf>) -> Vec<PhotoMetadata> {
    paths
        .into_iter()
        .map(|path| (path.clone(), extract(&path)))
        .filter_map(|(path, result)| match result {
            Ok(value) => Some((path, value)),
            Err(e) => {
                eprintln!("Error: {}", e);
                None
            }
        })
        .map(|(path, dt)| PhotoMetadata { path, datetime: dt })
        .collect()
}
