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

#[cfg(test)]
mod tests {
    use super::*;

    mod mode_from_str {
        use super::*;

        #[test]
        fn test_valid_daily() {
            assert!(matches!(Mode::from_str("daily"), Ok(Mode::Daily)));
        }

        #[test]
        fn test_valid_monthly() {
            assert!(matches!(Mode::from_str("monthly"), Ok(Mode::Monthly)));
        }

        #[test]
        fn test_valid_compact() {
            assert!(matches!(Mode::from_str("compact"), Ok(Mode::Compact)));
        }

        #[test]
        fn test_invalid_case() {
            let result = Mode::from_str("Daily");
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), "Error: 'Daily' is not a valid mode. Valid modes: daily, monthly, compact");
        }

        #[test]
        fn test_invalid_value() {
            let result = Mode::from_str("weekly");
            assert!(result.is_err());
            assert!(result.unwrap_err().contains("weekly"));
        }

        #[test]
        fn test_empty_string() {
            let result = Mode::from_str("");
            assert!(result.is_err());
        }
    }

    mod action_from_str {
        use super::*;

        #[test]
        fn test_valid_move() {
            assert!(matches!(Action::from_str("move"), Ok(Action::Move)));
        }

        #[test]
        fn test_valid_copy() {
            assert!(matches!(Action::from_str("copy"), Ok(Action::Copy)));
        }

        #[test]
        fn test_invalid_case() {
            let result = Action::from_str("Move");
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), "Error: 'Move' is not a valid action. Valid actions: move or copy")
        }

        #[test]
        fn test_invalid_value() {
            let result = Action::from_str("delete");
            assert!(result.is_err());
            assert!(result.unwrap_err().contains("delete"));
        }

        #[test]
        fn test_empty_string() {
            let result = Action::from_str("");
            assert!(result.is_err());
        }
    }
}
