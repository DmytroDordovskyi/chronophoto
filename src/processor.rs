use crate::discovery::discover_images;
use crate::metadata::paths_to_metadata;
use crate::mover::move_multiple;
use crate::path_builder::calc_paths;
use crate::types::Args;
use env_logger::{Builder, Target};
use log::LevelFilter::{Debug, Info};
use log::info;
use std::fs::File;

pub fn process(args: Args) {
    init_logger(&args);

    let paths = discover_images(args.source.clone());
    info!("Found {} photos", paths.len());

    let path_pairs = calc_paths(paths_to_metadata(paths), &args);
    move_multiple(path_pairs, args.dry_run)
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
