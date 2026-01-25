use std::fs;
use std::path::{Path, PathBuf};

pub fn move_multiple(path_pairs: Vec<(PathBuf, PathBuf)>, dry_run: bool) {
    println!("Would organize {} photos", path_pairs.len());

    if dry_run {
        for (src, dst) in path_pairs.iter() {
            println!("{} -> {}", src.display(), dst.display());
        }
    } else {
        for (src, dst) in path_pairs.iter() {
            match move_one(src, dst) {
                Ok(pb) => println!("File moved from {} to {}", src.display(), pb.display()),
                Err(err) => eprintln!("Error during moving file: {}", err),
            }
        }
    }
}

fn move_one(source: &PathBuf, destination: &PathBuf) -> Result<PathBuf, std::io::Error> {
    let parent_dir = destination.parent().unwrap();
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
    let name = file_path.file_stem().unwrap().display().to_string();
    let ext = file_path.extension().unwrap().display().to_string();
    let mut counter = 1;

    let mut new_path = PathBuf::from(format!(
        "{}/{}({}).{}",
        parent_dir.display(),
        name,
        counter,
        ext
    ));

    while fs::exists(&new_path)? {
        counter += 1;
        new_path = PathBuf::from(format!(
            "{}/{}({}).{}",
            parent_dir.display(),
            name,
            counter,
            ext
        ));
    }

    Ok(new_path)
}
