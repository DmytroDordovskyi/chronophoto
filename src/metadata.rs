use crate::exif_extractor::extract_datetime;
use crate::types::PhotoMetadata;
use std::path::PathBuf;

pub fn paths_to_metadata(paths: Vec<PathBuf>) -> Vec<PhotoMetadata> {
    paths
        .into_iter()
        .map(|path| (path.clone(), extract_datetime(&path)))
        .filter_map(|(path, result)| match result {
            Ok(value) => Some((path, value)),
            Err(e) => {
                eprintln!("Error on extract exif from {}: {}", path.display(), e);
                None
            }
        })
        .map(|(path, dt)| PhotoMetadata { path, datetime: dt })
        .collect()
}
