use crate::types::PhotoDateTime;
use chrono::NaiveDate;
use exif::{DateTime, In, Reader, Tag, Value};
use std::path::Path;

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

pub fn extract(path: &Path) -> Result<PhotoDateTime, ExifError> {
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
