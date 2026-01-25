# Photo Library Specification

photo-library is a CLI tool to organize photos based on EXIF creation timestamp data.
The main purpose of the tool is to organize folders and move photos, placing each one in an appropriate folder.
Folders are created if they don't exist or reused if they already exist.

## Arguments

The tool accepts the following arguments:

```
photo-library <source> [OPTIONS]
```

**Positional Arguments:**
- `source` - Directory containing photos to organize (e.g., `.` for current directory)

**Options:**
1. `--mode` - Identifies folder structure pattern (based on EXIF timestamp)
2. `--limit` - Maximum photos per month for compact mode (default: 25)
3. `--library` - Root folder of the photo library
4. `--rename` - Boolean flag to determine if photos should be renamed according to EXIF timestamp
   - When present, photos are renamed to format: `YYYYMMDD_hhmmss.ext`
   - `YYYY` = 4-digit year, `MM` = 2-digit month, `DD` = 2-digit day
   - `hh` = 2-digit hour (24-hour format), `mm` = 2-digit minute, `ss` = 2-digit second
   - `ext` = original file extension (preserved)
   - Example: `20250629_143052.jpg` for a photo taken on June 29, 2025 at 2:30:52 PM
5. `--dry-run` - Boolean flag to preview operations without actually moving files
   - When present, displays all source → destination paths without modifying files
   - Useful for verifying the organization plan before executing
   - No files are moved or directories created in dry-run mode

## Usage Examples

```bash
# Preview what would happen (dry-run)
photo-library . --mode=compact --limit=20 --library=~/Pictures/my-photos --rename --dry-run

# Actually organize photos
photo-library . --mode=compact --limit=20 --library=~/Pictures/my-photos --rename

# Simple daily organization without renaming
photo-library ~/Downloads --mode=daily --library=~/Pictures/organized
```

## Organization Modes

The tool supports 3 modes of photo organization (folder structure):

### monthly
Organizes structure as `YYYY/MM` where:
- `YYYY` is the year the photo was taken
- `MM` is the corresponding month

Example: `2025/06` (folder "2025" with folder "06" inside it)

### daily (default)
Organizes structure as `YYYY/MM/DD` where:
- `YYYY` is the year the photo was taken
- `MM` is the corresponding month
- `DD` is the day

Example: `2025/06/08` (folder "2025", then "06" inside it, and "08" inside "06")

### compact
Uses the `limit` parameter (default: 25).
- Organizes as daily structure if the month has more than `limit` photos
- Organizes as monthly structure otherwise

## Supported File Formats

The tool processes all standard image formats that contain EXIF data:
- **Primary formats**: JPG, JPEG, PNG
- **Extended support**: Any image format supported by the EXIF library (TIFF, BMP, WEBP, etc.)
- The same processing code handles all formats without format-specific restrictions

## Error Handling and Edge Cases

### Missing EXIF Data
- Photos without EXIF creation timestamp are logged to error log with full file path
- These photos are skipped during organization process
- Error log entry format: `ERROR: Missing EXIF data - /full/path/to/photo.jpg`

### Filename Conflicts
When multiple photos would result in the same filename (due to duplicate timestamps or existing files), the tool automatically resolves conflicts by adding numerical suffixes:

- First conflict: `20250629_143052(1).jpg`
- Second conflict: `20250629_143052(2).jpg`
- Pattern continues: `(3)`, `(4)`, etc.

This applies to both:
- Multiple photos with identical timestamps
- Existing files in the destination directory
