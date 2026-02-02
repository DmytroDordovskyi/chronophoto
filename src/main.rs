use chronophoto::processor::process;
use chronophoto::types::Args;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CliArgs {
    /// Directory containing photos to organize
    source: PathBuf,

    /// Root folder of the photo library
    library: PathBuf,

    /// Folder structure pattern: daily, monthly, compact, or flat (no subfolders)
    #[arg(short, long, default_value = "daily")]
    mode: String,

    /// Maximum photos per month for compact mode
    #[arg(short = 'n', long, default_value_t = 25)]
    limit: u16,

    /// Rename files to YYYYMMDD_hhmmss format
    #[arg(short, long, default_value_t = false)]
    rename: bool,

    /// File operation: move or copy
    #[arg(short, long, default_value = "move")]
    action: String,

    /// Preview changes without modifying files
    #[arg(long, default_value_t = false)]
    dry_run: bool,

    /// Path to write log file
    #[arg(short, long)]
    log_file: Option<PathBuf>,

    /// Enable verbose logging
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

impl TryFrom<CliArgs> for Args {
    type Error = String;

    fn try_from(cli: CliArgs) -> Result<Self, Self::Error> {
        Ok(Args {
            source: cli.source,
            mode: cli.mode.parse()?,
            action: cli.action.parse()?,
            library: cli.library,
            limit: cli.limit,
            rename: cli.rename,
            log_file: cli.log_file,
            dry_run: cli.dry_run,
            verbose: cli.verbose,
        })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CliArgs::parse();
    let has_log_file = args.log_file.is_some();

    match process(args.try_into()?) {
        Ok(summary) => {
            if has_log_file {
                println!("{}", summary);
            }
            Ok(())
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
