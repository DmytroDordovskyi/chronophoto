use crate::types::Args;
use env_logger::{Builder, Target};
use log::LevelFilter::{Debug, Info};
use std::fs::{self, File, OpenOptions};
use std::path::Path;

pub fn init_logger(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    let level = if args.verbose || args.dry_run {
        Debug
    } else {
        Info
    };

    if let Some(path) = &args.log_file {
        match File::create(path) {
            Ok(log_file) => {
                let _ = Builder::from_default_env()
                    .target(Target::Pipe(Box::new(log_file)))
                    .filter_level(level)
                    .try_init();
            }
            Err(e) => return Err(Box::new(e)),
        }
    } else {
        let _ = Builder::from_default_env().filter_level(level).try_init();
    }

    Ok(())
}

pub fn need_progress_bar(args: &Args) -> bool {
    !args.dry_run && args.log_file.is_some()
}

pub fn validate_io_dirs(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
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
