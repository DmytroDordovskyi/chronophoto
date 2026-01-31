use indicatif::ProgressBar;
use log::{debug, error, info};
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use crate::types::Action;

pub fn transfer_multiple(
    path_pairs: Vec<(PathBuf, PathBuf)>,
    dry_run: bool,
    action: Action,
    progress_bar: &Option<ProgressBar>,
) -> (usize, usize) {
    info!("Will organize {} photos", path_pairs.len());

    if dry_run {
        for (src, dst) in path_pairs.iter() {
            debug!(
                "[DRY RUN] Would organize file from {} to {}",
                src.display(),
                dst.display()
            );
        }
        (path_pairs.len(), 0)
    } else {
        let mut transferred = 0;
        let mut failed = 0;

        for (src, dst) in path_pairs.iter() {
            match transfer_one(src, dst, action) {
                Ok(pb) => {
                    debug!(
                        "Successfully organized file from {} to {}",
                        src.display(),
                        pb.display()
                    );
                    transferred += 1;
                }
                Err(err) => {
                    error!("Failed to organize file {}: {}", src.display(), err);
                    failed += 1;
                }
            }
            if let Some(pb) = progress_bar {
                pb.inc(1);
            }
        }

        (transferred, failed)
    }
}

fn transfer_one(
    source: &PathBuf,
    destination: &PathBuf,
    action: Action,
) -> Result<PathBuf, std::io::Error> {
    let parent_dir = destination
        .parent()
        .expect("destination should have parent directory");
    fs::create_dir_all(parent_dir)?;

    let final_destination = if fs::exists(destination)? {
        next_available_name(destination, parent_dir)?
    } else {
        destination.to_path_buf()
    };

    match action {
        Action::Move => rename(source, final_destination),
        Action::Copy => {
            fs::copy(source, &final_destination)?;
            Ok(final_destination)
        }
    }
}

fn rename(source: &PathBuf, destination: PathBuf) -> Result<PathBuf, std::io::Error> {
    match fs::rename(source, &destination) {
        Ok(_) => Ok(destination),
        Err(e) if e.kind() == ErrorKind::CrossesDevices => {
            // Fallback: copy then delete
            fs::copy(source, &destination)?;
            fs::remove_file(source)?;
            Ok(destination)
        }
        Err(e) => Err(e),
    }
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
