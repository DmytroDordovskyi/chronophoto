use crate::discovery::discover_images;
use crate::metadata::paths_to_metadata;
use crate::mover::move_multiple;
use crate::path_builder::from_to_paths;
use crate::types::Args;
use env_logger::{Builder, Target};
use indicatif::{ProgressBar, ProgressStyle};
use log::LevelFilter::{Debug, Info};
use log::info;
use std::fs::{self, File, OpenOptions};
use std::path::Path;

pub fn process(args: Args) -> Result<String, Box<dyn std::error::Error>> {
    validate_io_dirs(&args)?;
    init_logger(&args)?;

    let paths = discover_images(args.source.clone());
    let all_files_count: usize = paths.len();

    let metadata_vec = paths_to_metadata(paths);

    let path_pairs = from_to_paths(metadata_vec, &args);
    let skipped = all_files_count - path_pairs.len();

    let with_progress_bar = need_progress_bar(&args);

    let pb = if with_progress_bar {
        Some(
            ProgressBar::new(path_pairs.len() as u64)
                .with_message("Moving files")
                .with_style(
                    ProgressStyle::default_bar()
                        .template("[{bar:40}] {pos}/{len} - {msg}")
                        .unwrap(),
                ),
        )
    } else {
        None
    };
    let (moved, failed) = move_multiple(path_pairs, args.dry_run, &pb);

    let summary = if args.dry_run {
        format!(
            "[DRY RUN] Processed {} files: {} would be moved, {} skipped (no EXIF)",
            all_files_count, moved, skipped
        )
    } else {
        format!(
            "Processed {} files: {} moved, {} skipped (no EXIF), {} failed",
            all_files_count, moved, skipped, failed
        )
    };

    if let Some(pb) = pb {
        pb.finish_with_message(summary.clone());
    } else {
        info!("{}", summary);
    }

    Ok(summary)
}

fn validate_io_dirs(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    if !args.source.exists() {
        return Err(format!("Source directory does not exist: {:?}", args.source).into());
    }

    if args.library.exists() && !is_dir_writable(&args.library) {
        return Err(format!(
            "Library directory exists but is not writable: {:?}",
            args.library
        )
        .into());
    }

    Ok(())
}

fn is_dir_writable(path: &Path) -> bool {
    let test_file = path.join(".writability_test");

    match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&test_file)
    {
        Ok(_) => {
            let _ = fs::remove_file(&test_file);
            true
        }
        Err(_) => false,
    }
}

fn init_logger(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    let level = if args.verbose || args.dry_run {
        Debug
    } else {
        Info
    };

    if let Some(path) = &args.log_file {
        match File::create(path) {
            Ok(log_file) => Builder::from_default_env()
                .target(Target::Pipe(Box::new(log_file)))
                .filter_level(level)
                .init(),
            Err(e) => return Err(Box::new(e)),
        }
    } else {
        Builder::from_default_env().filter_level(level).init();
    }

    Ok(())
}

fn need_progress_bar(args: &Args) -> bool {
    !args.dry_run && args.log_file.is_some()
}
