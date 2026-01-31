use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
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
            _ => Err(format!(
                "Error: '{}' is not a valid mode. Valid modes: daily, monthly, compact",
                s
            )),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Action {
    Move,
    Copy,
}

impl FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "move" => Ok(Self::Move),
            "copy" => Ok(Self::Copy),
            _ => Err(format!(
                "Error: '{}' is not a valid action. Valid actions: move or copy",
                s
            )),
        }
    }
}

pub struct Args {
    pub source: PathBuf,
    pub library: PathBuf,
    pub mode: Mode,
    pub limit: u16,
    pub rename: bool,
    pub action: Action,
    pub dry_run: bool,
    pub log_file: Option<PathBuf>,
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
