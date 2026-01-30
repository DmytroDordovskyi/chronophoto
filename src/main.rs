use clap::Parser;
use photo_library::processor::process;
use photo_library::types::Args;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CliArgs {
    source: PathBuf,
    #[arg(short, long, default_value = "daily")]
    mode: String,
    #[arg(short, long, default_value = "move")]
    action: String,
    #[arg(long)]
    library: PathBuf,
    #[arg(short = 'n', long, default_value_t = 25)]
    limit: u16,
    #[arg(short, long, default_value_t = false)]
    rename: bool,
    #[arg(short, long)]
    log_file: Option<PathBuf>,
    #[arg(long, default_value_t = false)]
    dry_run: bool,
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
