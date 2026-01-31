use crate::types::{Args, Mode, PhotoMetadata};
use std::collections::HashMap;
use std::path::PathBuf;

pub fn from_to_paths(metadata: Vec<PhotoMetadata>, args: &Args) -> Vec<(PathBuf, PathBuf)> {
    match args.mode {
        Mode::Daily => metadata
            .into_iter()
            .map(|md| build_daily_path(&md, args.library.clone(), args.rename))
            .collect(),
        Mode::Monthly => metadata
            .into_iter()
            .map(|md| build_monthly_path(&md, args.library.clone(), args.rename))
            .collect(),
        Mode::Compact => build_compact_paths(metadata, args),
    }
}

fn files_per_month(metadata: &[PhotoMetadata]) -> HashMap<String, u16> {
    metadata.iter().fold(HashMap::new(), |mut acc, md| {
        let key = group_key(md);
        match acc.get(&key) {
            Some(v) => acc.insert(key, v + 1),
            None => acc.insert(key, 1),
        };
        acc
    })
}

fn group_key(md: &PhotoMetadata) -> String {
    format!("{}-{}", md.datetime.year, md.datetime.month)
}

fn build_compact_paths(metadata: Vec<PhotoMetadata>, args: &Args) -> Vec<(PathBuf, PathBuf)> {
    let stats = files_per_month(&metadata);
    metadata
        .into_iter()
        .map(|md| {
            let key = group_key(&md);
            let quantity = *stats.get(&key).expect("Should be at least 1");
            if quantity > args.limit {
                build_daily_path(&md, args.library.clone(), args.rename)
            } else {
                build_monthly_path(&md, args.library.clone(), args.rename)
            }
        })
        .collect()
}

fn build_daily_path(md: &PhotoMetadata, library: PathBuf, rename: bool) -> (PathBuf, PathBuf) {
    let folder = format!(
        "{:04}/{:02}/{:02}",
        md.datetime.year, md.datetime.month, md.datetime.day
    );
    (md.path.clone(), build_path(md, library, rename, folder))
}

fn build_monthly_path(md: &PhotoMetadata, library: PathBuf, rename: bool) -> (PathBuf, PathBuf) {
    let folder = format!("{:04}/{:02}", md.datetime.year, md.datetime.month);
    (md.path.clone(), build_path(md, library, rename, folder))
}

fn build_path(md: &PhotoMetadata, library: PathBuf, rename: bool, folder: String) -> PathBuf {
    let file_name = build_filename(md, rename).expect("photo must have filename");
    library.join(folder).join(file_name)
}

fn build_filename(md: &PhotoMetadata, rename: bool) -> Result<String, String> {
    if rename {
        let mut file_name = format!(
            "{:04}{:02}{:02}_{:02}{:02}{:02}",
            md.datetime.year,
            md.datetime.month,
            md.datetime.day,
            md.datetime.hour,
            md.datetime.minute,
            md.datetime.second
        );
        if let Some(ext) = md.path.extension() {
            file_name += &format!(".{}", ext.display());
        };
        return Ok(file_name);
    } else if let Some(name) = md.path.file_name() {
        return Ok(name.display().to_string());
    };

    Err("No file name".to_string())
}
