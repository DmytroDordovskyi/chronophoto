use crate::discovery::discover_images;
use crate::metadata::paths_to_metadata;
use crate::mover::move_multiple;
use crate::path_builder::from_to_paths;
use crate::types::Args;
use env_logger::{Builder, Target};
use log::LevelFilter::{Debug, Info};
use log::info;
use std::fs::File;

pub fn process(args: Args) {
    init_logger(&args);

    let paths = discover_images(args.source.clone());
    let all_files_count: usize = paths.len();
    info!("Found {} photos", all_files_count);

    let metadata_vec = paths_to_metadata(paths);

    let path_pairs = from_to_paths(metadata_vec, &args);
    let skipped = all_files_count - path_pairs.len();

    let (moved, failed) = move_multiple(path_pairs, args.dry_run);

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

    info!("{}", summary);

    if args.log_file.is_some() {
        println!("{}", summary);
    }
}

fn init_logger(args: &Args) {
    let level = if args.verbose || args.dry_run {
        Debug
    } else {
        Info
    };

    if let Some(path) = &args.log_file {
        let log_file = File::create(path).expect("Failed to create log file");

        Builder::from_default_env()
            .target(Target::Pipe(Box::new(log_file)))
            .filter_level(level)
            .init();
    } else {
        Builder::from_default_env().filter_level(level).init();
    }
}
