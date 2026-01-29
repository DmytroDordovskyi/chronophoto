use log::{debug, error, info};
use std::fs;
use std::path::{Path, PathBuf};

pub fn move_multiple(path_pairs: Vec<(PathBuf, PathBuf)>, dry_run: bool) -> (usize, usize) {
    info!("Will organize {} photos", path_pairs.len());

    if dry_run {
        for (src, dst) in path_pairs.iter() {
            debug!(
                "[DRY RUN] Would move file from {} to {}",
                src.display(),
                dst.display()
            );
        }
        (path_pairs.len(), 0)
    } else {
        let mut moved = 0;
        let mut failed = 0;

        for (src, dst) in path_pairs.iter() {
            match move_one(src, dst) {
                Ok(pb) => {
                    debug!(
                        "Successfully moved file from {} to {}",
                        src.display(),
                        pb.display()
                    );
                    moved += 1;
                }
                Err(err) => {
                    error!("Failed to move file {}: {}", src.display(), err);
                    failed += 1;
                }
            }
        }

        (moved, failed)
    }
}

fn move_one(source: &PathBuf, destination: &PathBuf) -> Result<PathBuf, std::io::Error> {
    let parent_dir = destination
        .parent()
        .expect("destination should have parent directory");
    fs::create_dir_all(parent_dir)?;

    let final_destination = if fs::exists(destination)? {
        next_available_name(destination, parent_dir)?
    } else {
        destination.to_path_buf()
    };

    fs::rename(source, &final_destination)?;
    Ok(final_destination)
}

fn next_available_name<'a>(
    file_path: &'a Path,
    parent_dir: &'a Path,
) -> Result<PathBuf, std::io::Error> {
    let name = file_path
        .file_stem()
        .expect("destination path must have a filename stem")
        .display()
        .to_string();
    let ext = file_path
        .extension()
        .expect("destination path must have a file extension")
        .display()
        .to_string();
    let mut counter = 1;

    let mut new_path = parent_dir.join(format!("{}({}).{}", name, counter, ext));

    while fs::exists(&new_path)? {
        counter += 1;
        new_path = parent_dir.join(format!("{}({}).{}", name, counter, ext));
    }

    Ok(new_path)
}
