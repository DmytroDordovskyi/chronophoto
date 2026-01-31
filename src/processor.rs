use crate::discovery::discover_files;
use crate::metadata::paths_to_metadata;
use crate::organizer::from_to_paths;
use crate::setup::{init_logger, need_progress_bar, validate_io_dirs};
use crate::transfer::transfer_multiple;
use crate::types::Args;

use indicatif::{ProgressBar, ProgressStyle};

use log::info;

pub fn process(args: Args) -> Result<String, Box<dyn std::error::Error>> {
    validate_io_dirs(&args)?;
    init_logger(&args)?;

    let paths = discover_files(args.source.clone());
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
    let (transferred, failed) = transfer_multiple(path_pairs, args.dry_run, args.action, &pb);

    let summary = if args.dry_run {
        format!(
            "[DRY RUN] Processed {} files: {} would be transferred, {} skipped (no EXIF)",
            all_files_count, transferred, skipped
        )
    } else {
        format!(
            "Processed {} files: {} transferred, {} skipped (no EXIF), {} failed",
            all_files_count, transferred, skipped, failed
        )
    };

    if let Some(pb) = pb {
        pb.finish_with_message(summary.clone());
    } else {
        info!("{}", summary);
    }

    Ok(summary)
}
