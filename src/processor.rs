use crate::discovery::discover_images;
use crate::metadata::paths_to_metadata;
use crate::mover::move_multiple;
use crate::path_builder::calc_paths;
use crate::types::Args;

pub fn process(args: Args) {
    let paths = discover_images(args.source.clone());
    println!("Found {} images", paths.len());

    let path_pairs = calc_paths(paths_to_metadata(paths), &args);
    move_multiple(path_pairs, args.dry_run)
}
