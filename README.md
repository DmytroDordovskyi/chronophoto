# Chronophoto

A CLI tool to automatically organize your photo library based on EXIF timestamp data.

## Overview

Chronophoto helps you organize your photos by reading the creation timestamp from EXIF metadata and moving them into a structured folder hierarchy. Perfect for managing large photo collections from cameras, phones, and other devices.

## Features

- **Smart Organization**: Automatically creates folder structures based on photo timestamps
- **Multiple Modes**: Choose between daily, monthly, compact or flat organization
- **Flexible Operations**: Move or copy files to preserve originals
- **Auto-Rename**: Optionally rename files to timestamp format (YYYYMMDD_hhmmss)
- **Conflict Resolution**: Automatically handles duplicate filenames
- **Dry Run**: Preview changes before making them
- **Progress Tracking**: Visual progress bar for batch operations
- **Comprehensive Logging**: Optional log file output for tracking operations

## Installation

```bash
git clone https://github.com/DmytroDordovskyi/chronophoto.git
cd chronophoto
cargo build --release
```

The binary will be available at `target/release/chronophoto`.

## Usage

```bash
chronophoto <source> <library> [OPTIONS]
```

### Arguments

- `<source>` - Directory containing photos to organize
- `<library>` - Root folder of your photo library (where organized photos will go)

### Options

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--mode` | `-m` | Organization mode: `daily`, `monthly`, `compact` or `flat` | `daily` |
| `--limit` | `-n` | Max photos per month for compact mode | `25` |
| `--rename` | `-r` | Rename files to YYYYMMDD_hhmmss format | `false` |
| `--action` | `-a` | File operation: `move` or `copy` | `move` |
| `--dry-run` | | Preview changes without modifying files | `false` |
| `--log-file` | `-l` | Path to write log file | None |
| `--verbose` | `-v` | Enable verbose logging | `false` |

## Organization Modes

### Daily Mode (Default)
Creates a `YYYY/MM/DD` folder structure:
```
library/
├── 2025/
│   ├── 01/
│   │   ├── 15/
│   │   │   ├── photo1.jpg
│   │   │   └── photo2.jpg
│   │   └── 16/
│   │       └── photo3.jpg
```

### Monthly Mode
Creates a `YYYY/MM` folder structure:
```
library/
├── 2025/
│   ├── 01/
│   │   ├── photo1.jpg
│   │   ├── photo2.jpg
│   │   └── photo3.jpg
```

### Compact Mode
Intelligently switches between daily and monthly based on photo count:
- **Monthly structure** if month has ≤ limit photos (default: 25)
- **Daily structure** if month has > limit photos

### Flat Mode
Transfers all photos to library root. **Best used with `--rename`** for chronological sorting.
```
library/
   ├── 20251217_001122.jpg
   ├── 20260129_112233.jpg
   └── 20260131_223344.jpg
```
   
## Examples

### Preview organization (recommended first step)
```bash
chronophoto ~/Downloads ~/Pictures/Library --dry-run
```

### Organize with daily structure
```bash
chronophoto ~/Downloads ~/Pictures/Library --mode daily
```

### Organize and rename files
```bash
chronophoto ~/Downloads ~/Pictures/Library --mode daily --rename
```

### Copy instead of move (preserve originals)
```bash
chronophoto ~/Downloads ~/Pictures/Library --action copy
```

### Compact mode with custom limit
```bash
chronophoto ~/Downloads ~/Pictures/Library --mode compact --limit 50
```

### Full example with logging
```bash
chronophoto ~/Downloads ~/Pictures/Library \
  --mode compact \
  --limit 30 \
  --rename \
  --action copy \
  --log-file ~/chronophoto.log \
  --verbose
```

## Supported File Formats

Chronophoto supports common image formats that contain EXIF metadata, including JPEG, PNG, TIFF, HEIC/HEIF, and WebP. Photos without valid EXIF timestamp data will be skipped and logged.

## File Naming

When using the `--rename` flag, files are renamed to:
```
YYYYMMDD_hhmmss.ext
```

Where:
- `YYYY` = 4-digit year
- `MM` = 2-digit month
- `DD` = 2-digit day
- `hh` = hour (24-hour format)
- `mm` = minute
- `ss` = second
- `ext` = original file extension

Example: `20250129_143052.jpg`

### Duplicate Handling

If multiple photos have the same timestamp or a file already exists, chronophoto automatically adds a suffix:
- `20250129_143052(1).jpg`
- `20250129_143052(2).jpg`
- And so on...

## Error Handling

Photos without EXIF timestamp data are:
- Skipped during organization
- Logged to the error log (if `--log-file` is specified)
- Listed with their full file path for manual handling

## Tips

1. **Always start with `--dry-run`** to preview what will happen
2. **Use `--action copy`** if you want to keep originals untouched
3. **Enable `--verbose`** and `--log-file`** for troubleshooting
4. **Choose compact mode** for mixed-density photo collections
5. **Use `--rename`** for consistent, sortable filenames

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

## Author

Dmytro Dordovskyi
