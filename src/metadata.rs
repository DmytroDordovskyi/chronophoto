use crate::types::{PhotoDateTime, PhotoMetadata};
use chrono::NaiveDate;
use exif::{DateTime, In, Reader, Tag, Value};
use log::warn;
use std::path::{Path, PathBuf};

impl From<DateTime> for PhotoDateTime {
    fn from(dt: DateTime) -> Self {
        Self {
            year: dt.year,
            month: dt.month,
            day: dt.day,
            hour: dt.hour,
            minute: dt.minute,
            second: dt.second,
        }
    }
}

#[derive(Debug)]
pub enum ExifError {
    Io(std::io::Error),
    ReadData(exif::Error),
    ParseDateError(exif::Error),
    NoDataError,
}

impl From<std::io::Error> for ExifError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<exif::Error> for ExifError {
    fn from(err: exif::Error) -> Self {
        Self::ReadData(err)
    }
}

impl std::fmt::Display for ExifError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Io(io_error) => write!(f, "{}", io_error),
            Self::ReadData(exif_error) => write!(f, "{}", exif_error),
            Self::ParseDateError(exif_error) => write!(f, "{}", exif_error),
            Self::NoDataError => write!(f, "No DateTime field in EXIF data"),
        }
    }
}

pub fn paths_to_metadata(paths: Vec<PathBuf>) -> Vec<PhotoMetadata> {
    paths
        .into_iter()
        .map(|path| (path.clone(), extract_datetime(&path)))
        .filter_map(|(path, result)| match result {
            Ok(value) => Some((path, value)),
            Err(e) => {
                warn!(
                    "Failed to extract EXIF metadata from {}: {}",
                    path.display(),
                    e
                );
                None
            }
        })
        .map(|(path, dt)| PhotoMetadata { path, datetime: dt })
        .collect()
}

fn extract_datetime(path: &Path) -> Result<PhotoDateTime, ExifError> {
    let file = std::fs::File::open(path)?;
    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = Reader::new();
    let exif = exifreader.read_from_container(&mut bufreader)?;
    if let Some(field) = exif.get_field(Tag::DateTime, In::PRIMARY) {
        match field.value {
            Value::Ascii(ref vec) if !vec.is_empty() => match DateTime::from_ascii(&vec[0]) {
                Ok(dt) if is_valid_datetime(&dt) => return Ok(dt.into()),
                Err(err) => return Err(ExifError::ParseDateError(err)),
                _ => return Err(ExifError::NoDataError),
            },
            _ => (),
        }
    }
    Err(ExifError::NoDataError)
}

fn is_valid_datetime(dt: &DateTime) -> bool {
    if dt.year < 1970 {
        return false;
    }

    NaiveDate::from_ymd_opt(dt.year as i32, dt.month as u32, dt.day as u32)
        .and_then(|date| date.and_hms_opt(dt.hour as u32, dt.minute as u32, dt.second as u32))
        .is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod is_valid_datetime {
        use super::*;

        #[test]
        fn test_valid_datetime() {
            let dt = DateTime {
                year: 2026,
                month: 6,
                day: 15,
                hour: 14,
                minute: 30,
                second: 45,
                nanosecond: None,
                offset: None,
            };
            assert!(is_valid_datetime(&dt));
        }

        #[test]
        fn test_year_outdated() {
            let dt = DateTime {
                year: 1969,
                month: 12,
                day: 31,
                hour: 0,
                minute: 0,
                second: 0,
                nanosecond: None,
                offset: None,
            };
            assert!(!is_valid_datetime(&dt));
        }

        #[test]
        fn test_boundary_year() {
            let dt = DateTime {
                year: 1970,
                month: 1,
                day: 1,
                hour: 0,
                minute: 0,
                second: 0,
                nanosecond: None,
                offset: None,
            };
            assert!(is_valid_datetime(&dt));
        }

        #[test]
        fn test_month_overflow() {
            let dt = DateTime {
                year: 2026,
                month: 13,
                day: 1,
                hour: 0,
                minute: 0,
                second: 0,
                nanosecond: None,
                offset: None,
            };
            assert!(!is_valid_datetime(&dt));
        }

        #[test]
        fn test_month_zero() {
            let dt = DateTime {
                year: 2026,
                month: 0,
                day: 15,
                hour: 0,
                minute: 0,
                second: 0,
                nanosecond: None,
                offset: None,
            };
            assert!(!is_valid_datetime(&dt));
        }

        #[test]
        fn test_day_overflow() {
            let dt = DateTime {
                year: 2026,
                month: 2,
                day: 30,
                hour: 0,
                minute: 0,
                second: 0,
                nanosecond: None,
                offset: None,
            };
            assert!(!is_valid_datetime(&dt));
        }

        #[test]
        fn test_leap_year_valid() {
            let dt = DateTime {
                year: 2024,
                month: 2,
                day: 29,
                hour: 0,
                minute: 0,
                second: 0,
                nanosecond: None,
                offset: None,
            };
            assert!(is_valid_datetime(&dt));
        }

        #[test]
        fn test_leap_year_invalid() {
            let dt = DateTime {
                year: 2023,
                month: 2,
                day: 29,
                hour: 0,
                minute: 0,
                second: 0,
                nanosecond: None,
                offset: None,
            };
            assert!(!is_valid_datetime(&dt));
        }

        #[test]
        fn test_hour_overflow() {
            let dt = DateTime {
                year: 2026,
                month: 6,
                day: 15,
                hour: 24,
                minute: 0,
                second: 0,
                nanosecond: None,
                offset: None,
            };
            assert!(!is_valid_datetime(&dt));
        }

        #[test]
        fn test_minute_overflow() {
            let dt = DateTime {
                year: 2026,
                month: 6,
                day: 15,
                hour: 12,
                minute: 60,
                second: 0,
                nanosecond: None,
                offset: None,
            };
            assert!(!is_valid_datetime(&dt));
        }

        #[test]
        fn test_second_overflow() {
            let dt = DateTime {
                year: 2026,
                month: 6,
                day: 15,
                hour: 12,
                minute: 30,
                second: 60,
                nanosecond: None,
                offset: None,
            };
            assert!(!is_valid_datetime(&dt));
        }
    }
}
