# nomoji

> [!WARNING]
> AI SLOP ALERT

**nomoji** remove emoji characters from text files while preserving all other Unicode content.

## Features

- **Remove emojis** from single or multiple files
- **Preserve Unicode** - Keeps all non-emoji characters including accented letters, CJK scripts, Arabic, Hebrew, etc.
- **Multiple processing modes** - Output to stdout, edit in-place, or create backups
- **Dry-run mode** - Count emojis without removing them
- **Stdin support** - Process text piped from other commands
- **Detailed reporting** - Shows number of emojis removed per file and total summary
- **Comprehensive emoji coverage** - Handles emoticons, symbols, pictographs, flags, skin tone modifiers, and more

## Installation

### From Source

Requires [Rust](https://rust-lang.org) 1.85 or later:

```bash
git clone https://github.com/brianshumate/nomoji
cd nomoji
cargo build --release
```

The binary will be available at `target/release/nomoji`.

### Prerequisites

- Rust 1.85+ (for building from source)

## Usage

### Basic Usage

Process a file and output to stdout:

```bash
nomoji file.txt
```

### Process Multiple Files

```bash
nomoji file1.txt file2.txt file3.txt
```

### Edit Files In-Place

Remove emojis and save changes directly to the file:

```bash
nomoji -i file.txt
# or
nomoji --inplace file.txt
```

### Create Backups

Create a `.bak` backup of the original file before processing:

```bash
nomoji -b file.txt
# or
nomoji --backup file.txt
```

### Dry Run (Count Only)

Count emojis without removing them:

```bash
nomoji --dry-run file.txt
```

### Process from Stdin

```bash
echo "Hello 😀 World 🌍" | nomoji -
cat file.txt | nomoji -
```

### Combining Options

Create backups and edit in-place:

```bash
nomoji -b -i file.txt
```

## Examples

### Example 1: Clean up log files

```bash
nomoji -i application.log
```

### Example 2: Process multiple markdown files

```bash
nomoji -b *.md
```

### Example 3: Remove emojis from piped output

```bash
curl -s https://api.example.com/data | nomoji - | jq '.'
```

### Example 4: Preview changes before applying

```bash
nomoji --dry-run important.txt
```

## Command-Line Options

| Option | Short | Description |
|--------|-------|-------------|
| `--backup` | `-b` | Create backup files with `.bak` extension |
| `--inplace` | `-i` | Edit files in place |
| `--dry-run` | | Count emojis without removing them |
| `--help` | `-h` | Show help message |
| `--version` | `-V` | Show version information |

## Supported Emoji Types

nomoji recognizes and removes:

- **Emoticons** (😀 😃 😄 😂 🤣 etc.)
- **Symbols & Pictographs** (❤️ ♦️ ♠️ 💯 💢 etc.)
- **Transport & Map** (🚗 🚕 🚙 🚌 etc.)
- **Flags** (🇺🇸 🇬🇧 🇯🇵 etc.)
- **Skin Tone Modifiers** (👋🏻 👋🏼 👋🏽 👋🏾 👋🏿)
- **Food & Drink** (🍏 🍎 🍐 🍊 etc.)
- **Activities** (⚽ 🏀 🏈 ⚾ etc.)
- **Objects** (👓 🕶️ 🥽 🥼 etc.)
- **Geometric Shapes** (🔴 🔵 ⚪ ⚫ etc.)
- **Dingbats** (✅ ✔ ✖ ➕ ➖ etc.)
- **Copyright/Trademark** (© ® ™)

## What Gets Preserved

nomoji **only** removes emoji characters. All other content is preserved:

- Accented characters (café, résumé, naïve)
- CJK scripts (日本語, 中文)
- Arabic (العربية)
- Hebrew (עברית)
- Cyrillic (русский)
- Mathematical symbols
- All other Unicode text

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success - all files processed successfully |
| 1 | Error - one or more files failed to process |

## Sample Output

```
=== nomoji Report ===
Files processed: 3
Successful: 3
Total emojis found: 42

Per-file results:
  file1.txt: 15 emojis removed
  file2.txt: 20 emojis removed
  file3.txt: 7 emojis removed
```

## Testing

Run the test suite:

```bash
cargo test
```

## License

BSD 2-Clause License - see [LICENSE](LICENSE) file for details.
