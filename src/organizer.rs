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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::types::{Action, PhotoDateTime};

    fn create_test_args(mode: Mode, rename: bool, limit: u16) -> Args {
        Args {
            source: PathBuf::from("source"),
            library: PathBuf::from("test_dir"),
            mode,
            limit,
            rename,
            action: Action::Move,
            dry_run: false,
            log_file: None,
            verbose: false,
        }
    }

    fn create_test_metadata() -> Vec<PhotoMetadata> {
        vec![
            PhotoMetadata {
                path: PathBuf::from("photo1.png"),
                datetime: PhotoDateTime {
                    year: 2025,
                    month: 12,
                    day: 20,
                    hour: 14,
                    minute: 15,
                    second: 30,
                },
            },
            PhotoMetadata {
                path: PathBuf::from("photo2.jpg"),
                datetime: PhotoDateTime {
                    year: 2026,
                    month: 2,
                    day: 1,
                    hour: 10,
                    minute: 30,
                    second: 45,
                },
            },
            PhotoMetadata {
                path: PathBuf::from("photo3.gif"),
                datetime: PhotoDateTime {
                    year: 2026,
                    month: 2,
                    day: 1,
                    hour: 8,
                    minute: 0,
                    second: 0,
                },
            },
            PhotoMetadata {
                path: PathBuf::from("photo4.png"),
                datetime: PhotoDateTime {
                    year: 2026,
                    month: 2,
                    day: 20,
                    hour: 14,
                    minute: 15,
                    second: 30,
                },
            },
        ]
    }

    #[test]
    fn test_daily_mode_with_rename() {
        let args = create_test_args(Mode::Daily, true, 100);
        let metadata = create_test_metadata();
        let result = from_to_paths(metadata, &args);

        assert_eq!(result.len(), 4);
        assert_eq!(result[0].0, PathBuf::from("photo1.png"));
        assert_eq!(
            result[0].1,
            PathBuf::from("test_dir/2025/12/20/20251220_141530.png")
        );
        assert_eq!(result[1].0, PathBuf::from("photo2.jpg"));
        assert_eq!(
            result[1].1,
            PathBuf::from("test_dir/2026/02/01/20260201_103045.jpg")
        );
        assert_eq!(result[2].0, PathBuf::from("photo3.gif"));
        assert_eq!(
            result[2].1,
            PathBuf::from("test_dir/2026/02/01/20260201_080000.gif")
        );
        assert_eq!(result[3].0, PathBuf::from("photo4.png"));
        assert_eq!(
            result[3].1,
            PathBuf::from("test_dir/2026/02/20/20260220_141530.png")
        );
    }

    #[test]
    fn test_daily_mode_without_rename() {
        let args = create_test_args(Mode::Daily, false, 100);
        let metadata = create_test_metadata();
        let result = from_to_paths(metadata, &args);

        assert_eq!(result.len(), 4);
        assert_eq!(result[0].0, PathBuf::from("photo1.png"));
        assert_eq!(result[0].1, PathBuf::from("test_dir/2025/12/20/photo1.png"));
        assert_eq!(result[1].0, PathBuf::from("photo2.jpg"));
        assert_eq!(result[1].1, PathBuf::from("test_dir/2026/02/01/photo2.jpg"));
        assert_eq!(result[2].0, PathBuf::from("photo3.gif"));
        assert_eq!(result[2].1, PathBuf::from("test_dir/2026/02/01/photo3.gif"));
        assert_eq!(result[3].0, PathBuf::from("photo4.png"));
        assert_eq!(result[3].1, PathBuf::from("test_dir/2026/02/20/photo4.png"));
    }

    #[test]
    fn test_monthly_mode_with_rename() {
        let args = create_test_args(Mode::Monthly, true, 100);
        let metadata = create_test_metadata();
        let result = from_to_paths(metadata, &args);

        assert_eq!(result.len(), 4);
        assert_eq!(result[0].0, PathBuf::from("photo1.png"));
        assert_eq!(
            result[0].1,
            PathBuf::from("test_dir/2025/12/20251220_141530.png")
        );
        assert_eq!(result[1].0, PathBuf::from("photo2.jpg"));
        assert_eq!(
            result[1].1,
            PathBuf::from("test_dir/2026/02/20260201_103045.jpg")
        );
        assert_eq!(result[2].0, PathBuf::from("photo3.gif"));
        assert_eq!(
            result[2].1,
            PathBuf::from("test_dir/2026/02/20260201_080000.gif")
        );
        assert_eq!(result[3].0, PathBuf::from("photo4.png"));
        assert_eq!(
            result[3].1,
            PathBuf::from("test_dir/2026/02/20260220_141530.png")
        );
    }

    #[test]
    fn test_monthly_mode_without_rename() {
        let args = create_test_args(Mode::Monthly, false, 100);
        let metadata = create_test_metadata();
        let result = from_to_paths(metadata, &args);

        assert_eq!(result.len(), 4);
        assert_eq!(result[0].0, PathBuf::from("photo1.png"));
        assert_eq!(result[0].1, PathBuf::from("test_dir/2025/12/photo1.png"));
        assert_eq!(result[1].0, PathBuf::from("photo2.jpg"));
        assert_eq!(result[1].1, PathBuf::from("test_dir/2026/02/photo2.jpg"));
        assert_eq!(result[2].0, PathBuf::from("photo3.gif"));
        assert_eq!(result[2].1, PathBuf::from("test_dir/2026/02/photo3.gif"));
        assert_eq!(result[3].0, PathBuf::from("photo4.png"));
        assert_eq!(result[3].1, PathBuf::from("test_dir/2026/02/photo4.png"));
    }

    #[test]
    fn test_compact_mode_under_limit() {
        let args = create_test_args(Mode::Compact, true, 100);
        let metadata = create_test_metadata();
        let result = from_to_paths(metadata, &args);

        assert_eq!(result.len(), 4);
        assert_eq!(
            result[0].1,
            PathBuf::from("test_dir/2025/12/20251220_141530.png")
        );
        assert_eq!(
            result[1].1,
            PathBuf::from("test_dir/2026/02/20260201_103045.jpg")
        );
        assert_eq!(
            result[2].1,
            PathBuf::from("test_dir/2026/02/20260201_080000.gif")
        );
        assert_eq!(
            result[3].1,
            PathBuf::from("test_dir/2026/02/20260220_141530.png")
        );
    }

    #[test]
    fn test_compact_mode_over_limit() {
        let metadata = create_test_metadata();
        let args = create_test_args(Mode::Compact, true, 2);
        let result = from_to_paths(metadata, &args);

        assert_eq!(result.len(), 4);
        assert_eq!(
            result[0].1,
            PathBuf::from("test_dir/2025/12/20251220_141530.png")
        );
        assert_eq!(
            result[1].1,
            PathBuf::from("test_dir/2026/02/01/20260201_103045.jpg")
        );
        assert_eq!(
            result[2].1,
            PathBuf::from("test_dir/2026/02/01/20260201_080000.gif")
        );
        assert_eq!(
            result[3].1,
            PathBuf::from("test_dir/2026/02/20/20260220_141530.png")
        );
    }

    #[test]
    fn test_compact_mode_without_rename() {
        let metadata = create_test_metadata();
        let args = create_test_args(Mode::Compact, false, 2);
        let result = from_to_paths(metadata, &args);

        assert_eq!(result.len(), 4);
        assert_eq!(result[0].1, PathBuf::from("test_dir/2025/12/photo1.png"));
        assert_eq!(result[1].1, PathBuf::from("test_dir/2026/02/01/photo2.jpg"));
        assert_eq!(result[2].1, PathBuf::from("test_dir/2026/02/01/photo3.gif"));
        assert_eq!(result[3].1, PathBuf::from("test_dir/2026/02/20/photo4.png"));
    }
}
