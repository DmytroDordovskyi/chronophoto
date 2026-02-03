use chronophoto::processor::process;
use chronophoto::types::{Action, Args, Mode};
use std::fs;
use std::path::PathBuf;
use tempfile::{TempDir, tempdir};

fn setup_dirs() -> (TempDir, TempDir) {
    (tempdir().unwrap(), tempdir().unwrap())
}

fn copy_fixture(fixture_name: &str, dest: PathBuf) {
    let src = format!("tests/fixtures/{}", fixture_name);
    fs::copy(&src, dest).unwrap();
}

fn create_args(source: PathBuf, library: PathBuf) -> Args {
    // Create temp log file to suppress console output during tests
    let log_file = tempfile::NamedTempFile::new().unwrap();
    Args {
        source,
        library,
        mode: Mode::Daily,
        limit: 25,
        rename: false,
        action: Action::Move,
        dry_run: false,
        log_file: Some(log_file.path().to_path_buf()),
        verbose: false,
    }
}

#[test]
fn test_daily_default() {
    let (temp_source, temp_library) = setup_dirs();

    fs::create_dir_all(temp_source.path().join("qqq")).unwrap();
    copy_fixture(
        "photo_2025_06_15.jpg",
        temp_source.path().join("photo1.jpg"),
    );
    copy_fixture("photo_no_extension", temp_source.path().join("qqq/photo2"));

    let args = create_args(
        temp_source.path().to_path_buf(),
        temp_library.path().to_path_buf(),
    );

    let result = process(args);
    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        "Processed 2 files: 2 transferred, 0 were already organized, 0 skipped (no EXIF), 0 failed"
    );
    assert!(fs::exists(temp_library.path().join("2025/06/15/photo1.jpg")).unwrap());
    assert!(fs::exists(temp_library.path().join("2025/01/01/photo2")).unwrap());
}

#[test]
fn test_monthly() {
    let (temp_source, temp_library) = setup_dirs();

    fs::create_dir_all(temp_source.path().join("qqq")).unwrap();
    copy_fixture(
        "photo_2025_06_15.jpg",
        temp_source.path().join("photo1.jpg"),
    );
    copy_fixture("photo_no_extension", temp_source.path().join("qqq/photo2"));

    let mut args = create_args(
        temp_source.path().to_path_buf(),
        temp_library.path().to_path_buf(),
    );

    args.mode = Mode::Monthly;

    let result = process(args);
    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        "Processed 2 files: 2 transferred, 0 were already organized, 0 skipped (no EXIF), 0 failed"
    );
    assert!(fs::exists(temp_library.path().join("2025/06/photo1.jpg")).unwrap());
    assert!(fs::exists(temp_library.path().join("2025/01/photo2")).unwrap());
}

#[test]
fn test_compact_with_limit() {
    let (temp_source, temp_library) = setup_dirs();

    fs::create_dir_all(temp_source.path().join("qqq")).unwrap();
    copy_fixture(
        "photo_2025_06_15.jpg",
        temp_source.path().join("photo1.jpg"),
    );
    copy_fixture(
        "photo_2025_06_15.jpg",
        temp_source.path().join("photo2.jpg"),
    );
    copy_fixture(
        "photo_2025_06_15.jpg",
        temp_source.path().join("photo3.jpg"),
    );
    copy_fixture("photo_no_extension", temp_source.path().join("qqq/photo4"));
    copy_fixture("photo_no_extension", temp_source.path().join("photo5"));

    let mut args = create_args(
        temp_source.path().to_path_buf(),
        temp_library.path().to_path_buf(),
    );
    args.mode = Mode::Compact;
    args.limit = 2;

    let result = process(args);
    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        "Processed 5 files: 5 transferred, 0 were already organized, 0 skipped (no EXIF), 0 failed"
    );
    assert!(fs::exists(temp_library.path().join("2025/06/15/photo1.jpg")).unwrap());
    assert!(fs::exists(temp_library.path().join("2025/06/15/photo2.jpg")).unwrap());
    assert!(fs::exists(temp_library.path().join("2025/06/15/photo3.jpg")).unwrap());
    assert!(fs::exists(temp_library.path().join("2025/01/photo4")).unwrap());
    assert!(fs::exists(temp_library.path().join("2025/01/photo5")).unwrap());
}

#[test]
fn test_flat() {
    let (temp_source, temp_library) = setup_dirs();

    fs::create_dir_all(temp_source.path().join("qqq")).unwrap();
    copy_fixture(
        "photo_2025_06_15.jpg",
        temp_source.path().join("photo1.jpg"),
    );
    copy_fixture("photo_no_extension", temp_source.path().join("qqq/photo2"));

    let mut args = create_args(
        temp_source.path().to_path_buf(),
        temp_library.path().to_path_buf(),
    );
    args.mode = Mode::Flat;

    let result = process(args);
    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        "Processed 2 files: 2 transferred, 0 were already organized, 0 skipped (no EXIF), 0 failed"
    );
    assert!(fs::exists(temp_library.path().join("photo1.jpg")).unwrap());
    assert!(fs::exists(temp_library.path().join("photo2")).unwrap());
}

#[test]
fn test_rename() {
    let (temp_source, temp_library) = setup_dirs();

    copy_fixture(
        "photo_2025_06_15.jpg",
        temp_source.path().join("photo1.jpg"),
    );

    let mut args = create_args(
        temp_source.path().to_path_buf(),
        temp_library.path().to_path_buf(),
    );
    args.rename = true;

    let result = process(args);
    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        "Processed 1 files: 1 transferred, 0 were already organized, 0 skipped (no EXIF), 0 failed"
    );
    assert!(fs::exists(temp_library.path().join("2025/06/15/20250615_143000.jpg")).unwrap());
}

#[test]
fn test_copy() {
    let (temp_source, temp_library) = setup_dirs();

    copy_fixture(
        "photo_2025_06_15.jpg",
        temp_source.path().join("photo1.jpg"),
    );

    let mut args = create_args(
        temp_source.path().to_path_buf(),
        temp_library.path().to_path_buf(),
    );
    args.action = Action::Copy;

    let result = process(args);
    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        "Processed 1 files: 1 transferred, 0 were already organized, 0 skipped (no EXIF), 0 failed"
    );
    assert!(fs::exists(temp_library.path().join("2025/06/15/photo1.jpg")).unwrap());
    assert!(fs::exists(temp_source.path().join("photo1.jpg")).unwrap());
}

#[test]
fn test_move() {
    let (temp_source, temp_library) = setup_dirs();

    copy_fixture(
        "photo_2025_06_15.jpg",
        temp_source.path().join("photo1.jpg"),
    );

    let mut args = create_args(
        temp_source.path().to_path_buf(),
        temp_library.path().to_path_buf(),
    );
    args.action = Action::Move;

    let result = process(args);
    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        "Processed 1 files: 1 transferred, 0 were already organized, 0 skipped (no EXIF), 0 failed"
    );
    assert!(fs::exists(temp_library.path().join("2025/06/15/photo1.jpg")).unwrap());
    assert!(!fs::exists(temp_source.path().join("photo1.jpg")).unwrap());
}

#[test]
fn test_dry_run() {
    let (temp_source, temp_library) = setup_dirs();

    copy_fixture(
        "photo_2025_06_15.jpg",
        temp_source.path().join("photo1.jpg"),
    );

    let mut args = create_args(
        temp_source.path().to_path_buf(),
        temp_library.path().to_path_buf(),
    );
    args.dry_run = true;

    let result = process(args);
    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        "[DRY RUN] Processed 1 files: 1 would be transferred, 0 were already organized, 0 skipped (no EXIF)"
    );
    assert!(!fs::exists(temp_library.path().join("2025/06/15/photo1.jpg")).unwrap());
    assert!(fs::exists(temp_source.path().join("photo1.jpg")).unwrap());
}

#[test]
fn test_already_organized() {
    let (temp_source, temp_library) = setup_dirs();

    copy_fixture(
        "photo_2025_06_15.jpg",
        temp_source.path().join("photo1.jpg"),
    );
    let args = create_args(
        temp_source.path().to_path_buf(),
        temp_library.path().to_path_buf(),
    );

    let result = process(args);
    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        "Processed 1 files: 1 transferred, 0 were already organized, 0 skipped (no EXIF), 0 failed"
    );

    let args = create_args(
        temp_library.path().to_path_buf(),
        temp_library.path().to_path_buf(),
    );

    let result = process(args);
    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        "Processed 1 files: 0 transferred, 1 were already organized, 0 skipped (no EXIF), 0 failed"
    );
}

#[test]
fn test_invalid_exif_data() {
    let (temp_source, temp_library) = setup_dirs();

    copy_fixture("photo_no_exif.jpg", temp_source.path().join("no_exif.jpg"));
    copy_fixture(
        "photo_invalid_date.jpg",
        temp_source.path().join("invalid_date.jpg"),
    );

    let args = create_args(
        temp_source.path().to_path_buf(),
        temp_library.path().to_path_buf(),
    );

    let result = process(args);
    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        "Processed 2 files: 0 transferred, 0 were already organized, 2 skipped (no EXIF), 0 failed"
    );
    assert!(fs::read_dir(temp_library.path()).unwrap().next().is_none());
}

#[test]
fn test_name_conflict() {
    let (temp_source, temp_library) = setup_dirs();

    fs::create_dir_all(temp_source.path().join("qqq")).unwrap();
    copy_fixture(
        "photo_2025_06_15.jpg",
        temp_source.path().join("photo1.jpg"),
    );
    copy_fixture(
        "photo_2025_06_15.jpg",
        temp_source.path().join("qqq/photo1.jpg"),
    );

    let args = create_args(
        temp_source.path().to_path_buf(),
        temp_library.path().to_path_buf(),
    );

    let result = process(args);
    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        "Processed 2 files: 2 transferred, 0 were already organized, 0 skipped (no EXIF), 0 failed"
    );
    assert!(fs::exists(temp_library.path().join("2025/06/15/photo1.jpg")).unwrap());
    assert!(fs::exists(temp_library.path().join("2025/06/15/photo1(1).jpg")).unwrap());
}

#[test]
fn test_filename_conflict_on_rename() {
    let (temp_source, temp_library) = setup_dirs();

    copy_fixture(
        "photo_2025_06_15.jpg",
        temp_source.path().join("photo1.jpg"),
    );
    copy_fixture(
        "photo_2025_06_15.jpg",
        temp_source.path().join("photo2.jpg"),
    );

    let mut args = create_args(
        temp_source.path().to_path_buf(),
        temp_library.path().to_path_buf(),
    );
    args.rename = true;
    let result = process(args);

    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        "Processed 2 files: 2 transferred, 0 were already organized, 0 skipped (no EXIF), 0 failed"
    );
    assert!(fs::exists(temp_library.path().join("2025/06/15/20250615_143000.jpg")).unwrap());
    assert!(
        fs::exists(
            temp_library
                .path()
                .join("2025/06/15/20250615_143000(1).jpg")
        )
        .unwrap()
    );
}
