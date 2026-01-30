use indicatif::ProgressBar;
use log::{debug, error, info};
use std::fs;
use std::path::{Path, PathBuf};
use std::io::ErrorKind;

pub fn move_multiple(
    path_pairs: Vec<(PathBuf, PathBuf)>,
    dry_run: bool,
    progress_bar: &Option<ProgressBar>,
) -> (usize, usize) {
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
            if let Some(pb) = progress_bar {
                pb.inc(1);
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

    match fs::rename(source, &final_destination) {
        Ok(_) => Ok(final_destination),
        Err(e) if e.kind() == ErrorKind::CrossesDevices => {
            // Fallback: copy then delete
            fs::copy(source, &final_destination)?;
            fs::remove_file(source)?;
            Ok(final_destination)
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
