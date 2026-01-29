use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug)]
pub enum Mode {
    Daily,
    Monthly,
    Compact,
}

impl FromStr for Mode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "daily" => Ok(Self::Daily),
            "monthly" => Ok(Self::Monthly),
            "compact" => Ok(Self::Compact),
            _ => Err("Error: {}. Valid modes: daily, monthly, compact".to_string()),
        }
    }
}

pub struct Args {
    pub source: PathBuf,
    pub mode: Mode,
    pub library: PathBuf,
    pub limit: u16,
    pub rename: bool,
    pub log_file: Option<PathBuf>,
    pub dry_run: bool,
    pub verbose: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct PhotoDateTime {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

pub struct PhotoMetadata {
    pub path: PathBuf,
    pub datetime: PhotoDateTime,
}
