use clap::Parser;
use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(name = "nomoji")]
#[command(about = "Remove emoji characters from text files")]
#[command(version)]
#[command(arg_required_else_help = true)]
struct Args {
    /// Input file(s) to process (use - for stdin)
    files: Vec<String>,

    /// Create backup files with .bak extension
    #[arg(short, long)]
    backup: bool,

    /// Edit files in place
    #[arg(short, long)]
    inplace: bool,

    /// Count emojis without removing (dry run)
    #[arg(long)]
    dry_run: bool,
}

#[derive(Debug)]
struct ProcessResult {
    file: String,
    emojis_found: usize,
    success: bool,
    error: Option<String>,
}

fn is_emoji(c: char) -> bool {
    // Emoji ranges based on Unicode standard
    let code = c as u32;

    // Miscellaneous Symbols and Pictographs
    (0x1F300..=0x1F5FF).contains(&code)
        // Supplemental Symbols and Pictographs
        || (0x1F900..=0x1F9FF).contains(&code)
        // Emoticons
        || (0x1F600..=0x1F64F).contains(&code)
        // Transport and Map Symbols
        || (0x1F680..=0x1F6FF).contains(&code)
        // Miscellaneous Symbols
        || (0x2600..=0x26FF).contains(&code)
        // Dingbats
        || (0x2700..=0x27BF).contains(&code)
        // Enclosed Alphanumeric Supplement
        || (0x1F100..=0x1F1FF).contains(&code)
        // Enclosed Ideographic Supplement
        || (0x1F200..=0x1F2FF).contains(&code)
        // Geometric Shapes Extended
        || (0x1F780..=0x1F7FF).contains(&code)
        // Symbols and Pictographs Extended-A
        || (0x1FA00..=0x1FA6F).contains(&code)
        // Symbols and Pictographs Extended-B
        || (0x1FA70..=0x1FAFF).contains(&code)
        // Flags (regional indicators)
        || (0x1F1E6..=0x1F1FF).contains(&code)
        // Keycap sequences
        || code == 0x20E3
        // Zero Width Joiner for emoji sequences
        || code == 0x200D
        // Variation Selectors
        || (0xFE00..=0xFE0F).contains(&code)
        // Emoji modifier fitzpatrick
        || (0x1F3FB..=0x1F3FF).contains(&code)
        // Some additional symbols commonly used as emoji
        || matches!(code, 0x231A..=0x231B | 0x23E9..=0x23EC | 0x23F0 | 0x23F3 
            | 0x25FD..=0x25FE | 0x2614..=0x2615 | 0x2648..=0x2653 | 0x267F 
            | 0x2693 | 0x26A1 | 0x26AA..=0x26AB | 0x26BD..=0x26BE | 0x26C4..=0x26C5 
            | 0x26CE | 0x26D4 | 0x26EA | 0x26F2..=0x26F3 | 0x26F5 | 0x26FA 
            | 0x26FD | 0x2705 | 0x2728 | 0x274C | 0x274E | 0x2753..=0x2755 
            | 0x2795..=0x2797 | 0x27B0 | 0x27BF | 0x2B50 | 0x2B55 | 0x00A9 | 0x00AE 
            | 0x2122 | 0x3030 | 0x303D)
}

fn remove_emojis(input: &str) -> (String, usize) {
    let mut result = String::with_capacity(input.len());
    let mut count = 0;

    for c in input.chars() {
        if is_emoji(c) {
            count += 1;
        } else {
            result.push(c);
        }
    }

    (result, count)
}

fn read_input<P: AsRef<Path>>(path: P) -> io::Result<String> {
    fs::read_to_string(path)
}

fn write_output<P: AsRef<Path>>(path: P, content: &str) -> io::Result<()> {
    fs::write(path, content)
}

fn process_file(file: &str, args: &Args) -> ProcessResult {
    let result = match read_input(file) {
        Ok(content) => {
            let (cleaned, emoji_count) = remove_emojis(&content);

            if args.dry_run {
                ProcessResult {
                    file: file.to_string(),
                    emojis_found: emoji_count,
                    success: true,
                    error: None,
                }
            } else {
                let write_result = if args.backup {
                    let backup_path = format!("{}.bak", file);
                    if let Err(e) = fs::copy(file, &backup_path) {
                        ProcessResult {
                            file: file.to_string(),
                            emojis_found: emoji_count,
                            success: false,
                            error: Some(format!("Failed to create backup: {}", e)),
                        }
                    } else {
                        write_output(file, &cleaned)
                            .map(|_| ProcessResult {
                                file: file.to_string(),
                                emojis_found: emoji_count,
                                success: true,
                                error: None,
                            })
                            .unwrap_or_else(|e| ProcessResult {
                                file: file.to_string(),
                                emojis_found: emoji_count,
                                success: false,
                                error: Some(format!("Failed to write file: {}", e)),
                            })
                    }
                } else if args.inplace {
                    write_output(file, &cleaned)
                        .map(|_| ProcessResult {
                            file: file.to_string(),
                            emojis_found: emoji_count,
                            success: true,
                            error: None,
                        })
                        .unwrap_or_else(|e| ProcessResult {
                            file: file.to_string(),
                            emojis_found: emoji_count,
                            success: false,
                            error: Some(format!("Failed to write file: {}", e)),
                        })
                } else {
                    // Output to stdout
                    if let Err(e) = io::stdout().write_all(cleaned.as_bytes()) {
                        ProcessResult {
                            file: file.to_string(),
                            emojis_found: emoji_count,
                            success: false,
                            error: Some(format!("Failed to write to stdout: {}", e)),
                        }
                    } else {
                        ProcessResult {
                            file: file.to_string(),
                            emojis_found: emoji_count,
                            success: true,
                            error: None,
                        }
                    }
                };

                write_result
            }
        }
        Err(e) => ProcessResult {
            file: file.to_string(),
            emojis_found: 0,
            success: false,
            error: Some(format!("Failed to read file: {}", e)),
        },
    };

    result
}

fn process_stdin() -> io::Result<usize> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let (cleaned, count) = remove_emojis(&buffer);

    io::stdout().write_all(cleaned.as_bytes())?;

    Ok(count)
}

fn print_report(results: &[ProcessResult]) {
    let total_files = results.len();
    let successful = results.iter().filter(|r| r.success).count();
    let total_emojis: usize = results.iter().map(|r| r.emojis_found).sum();

    eprintln!("\n=== nomoji Report ===");
    eprintln!("Files processed: {}", total_files);
    eprintln!("Successful: {}", successful);

    if total_files != successful {
        eprintln!("Failed: {}", total_files - successful);
    }

    eprintln!("Total emojis found: {}", total_emojis);

    if !results.is_empty() {
        eprintln!("\nPer-file results:");
        for result in results {
            if let Some(ref error) = result.error {
                eprintln!(
                    "  {}: {} emojis - ERROR: {}",
                    result.file, result.emojis_found, error
                );
            } else {
                eprintln!("  {}: {} emojis removed", result.file, result.emojis_found);
            }
        }
    }
}

fn main() {
    let args = Args::parse();

    // If no files specified or "-" is used, read from stdin
    if args.files.is_empty() || (args.files.len() == 1 && args.files[0] == "-") {
        match process_stdin() {
            Ok(count) => {
                eprintln!("\n=== nomoji Report ===");
                eprintln!("Emojis removed from stdin: {}", count);
            }
            Err(e) => {
                eprintln!("Error reading from stdin: {}", e);
                std::process::exit(1);
            }
        }
        return;
    }

    let mut results = Vec::new();

    for file in &args.files {
        let result = process_file(file, &args);
        results.push(result);
    }

    print_report(&results);

    // Exit with error code if any file failed
    let failures = results.iter().filter(|r| !r.success).count();
    if failures > 0 {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_remove_emojis_basic() {
        let input = "Hello 😀 World 🌍!";
        let (result, count) = remove_emojis(input);
        assert_eq!(result, "Hello  World !");
        assert_eq!(count, 2);
    }

    #[test]
    fn test_no_emojis() {
        let input = "Hello World!";
        let (result, count) = remove_emojis(input);
        assert_eq!(result, "Hello World!");
        assert_eq!(count, 0);
    }

    #[test]
    fn test_unicode_preserved() {
        let input = "Café résumé naïve 日本語";
        let (result, count) = remove_emojis(input);
        assert_eq!(result, "Café résumé naïve 日本語");
        assert_eq!(count, 0);
    }

    #[test]
    fn test_mixed_content() {
        let input = "Test 🚀 rocket emoji 🔥 fire emoji";
        let (result, count) = remove_emojis(input);
        assert_eq!(result, "Test  rocket emoji  fire emoji");
        assert_eq!(count, 2);
    }

    #[test]
    fn test_empty_string() {
        let (result, count) = remove_emojis("");
        assert_eq!(result, "");
        assert_eq!(count, 0);
    }

    #[test]
    fn test_only_emojis() {
        let input = "😀🎉🚀🌍🔥";
        let (result, count) = remove_emojis(input);
        assert_eq!(result, "");
        assert_eq!(count, 5);
    }

    #[test]
    fn test_emoticons_range() {
        let input = "Faces: 😀😃😄😁😆😅😂🤣😊😇🙂🙃😉😌😍🥰😘😗😙😚😋😛😝😜🤪🤨🧐🤓😎🥸🤩🥳😏😒😞😔😟😕🙁☹️😣😖😫😩🥺😢😭😤😠😡🤬🤯😳🥵🥶😱😨😰😥😓🤗🤔🤭🤫🤥😶😐😑😬🙄😯😦😧😮😲🥱😴🤤😪😵🤐🥴🤢🤮🤧😷🤒🤕🤑🤠😈👿👹👺🤡💩👻💀☠️👽👾🤖🎃😺😸😹😻😼😽🙀😿😾";
        let (result, count) = remove_emojis(input);
        assert!(count > 50);
        assert!(!result.contains("😀"));
    }

    #[test]
    fn test_flags() {
        let input = "Flags: 🇺🇸🇬🇧🇯🇵🇫🇷🇩🇪";
        let (result, count) = remove_emojis(input);
        assert_eq!(count, 10);
        assert_eq!(result, "Flags: ");
    }

    #[test]
    fn test_skin_tone_modifiers() {
        let input = "People: 👋🏻👋🏼👋🏽👋🏾👋🏿";
        let (result, count) = remove_emojis(input);
        assert_eq!(count, 10);
        assert_eq!(result, "People: ");
    }

    #[test]
    fn test_symbols_and_pictographs() {
        let input = "Symbols: ♥️♦️♠️♣️💯💢💥💫💦💨🕳️💣💬👁️‍🗨️🗨️🗯️💭💤";
        let (result, count) = remove_emojis(input);
        assert!(count > 10);
        assert!(!result.contains("♥️"));
    }

    #[test]
    fn test_variation_selectors() {
        let input = "Text vs Emoji: #️⃣ *️⃣ 0️⃣ 1️⃣ 2️⃣";
        let (_result, count) = remove_emojis(input);
        assert!(count >= 5);
    }

    #[test]
    fn test_geometric_shapes() {
        let input = "Shapes: 🔴🔵⚪⚫🟥🟦🟧🟨🟩🟪⬛⬜◼️◻️🔶🔷🔸🔹";
        let (_result, count) = remove_emojis(input);
        assert!(count >= 10);
    }

    #[test]
    fn test_travel_and_places() {
        let input = "Travel: 🚗🚕🚙🚌🚎🏎️🚓🚑🚒🚐🛻🚚🚛🚜🦯🦽🦼🛴🚲🛵🏍️🛺🚨🚔🚍🚘🚖🚡🚠🚟🚃🚋🚞🚝🚄🚅🚈🚂🚆🚇🚊🚉✈️🛫🛬🛩️💺🛰️🚀🛸🚁🛶⛵🚤🛥️🛳️⛴️🚢⚓⛽🚧🚦🚥🚏🗺️🗿🗽🗼🏰🏯🏟️🎡🎢🎠⛲⛱️🏖️🏝️🏜️🌋⛰️🏔️🗻🏕️⛺🏠🏡🏘️🏚️🏗️🏭🏢🏬🏣🏤🏥🏦🏨🏪🏫🏩💒🏛️⛪🕌🕍🛕🕋⛩️🛤️🛣️🗾🎑🏞️🌅🌄🌠🎇🎆🌇🌆🏙️🌃🌌🌉🌁";
        let (_result, count) = remove_emojis(input);
        assert!(count > 50);
    }

    #[test]
    fn test_food_and_drink() {
        let input = "Food: 🍏🍎🍐🍊🍋🍌🍉🍇🍓🫐🍈🍒🍑🍍🥝🥑🍆🥔🥕🌽🌶️🫑🥒🥬🥦🧄🧅🍄🥜🌰🍞🥐🥖🥨🥯🥞🧇🧀🍖🍗🥩🥓🍔🍟🍕🌭🥪🌮🌯🫔🥙🧆🥚🍳🥘🍲🫕🥣🥗🍿🧈🧂🥫🍱🍘🍙🍚🍛🍜🍝🍠🍢🍣🍤🍥🥮🍡🥟🥠🥡🦀🦞🦐🦑🦪🍦🍧🍨🍩🍪🎂🍰🧁🥧🍫🍬🍭🍮🍯🍼🥛☕🫖🍵🍶🍾🍷🍸🍹🍺🍻🥂🥃🫗🥤🧋🧃🧉🧊";
        let (_result, count) = remove_emojis(input);
        assert!(count > 50);
    }

    #[test]
    fn test_activities() {
        let input = "Activities: ⚽🏀🏈⚾🥎🎾🏐🏉🥏🎱🪀🏓🏸🏒🏑🥍🏏🥅⛳🪁🏹🎣🤿🥊🥋🎽🛹🛼🛷⛸️🥌🎿⛷️🏂🪂🏋️‍♀️🏋️🏋️‍♂️🤼‍♀️🤼🤼‍♂️🤽‍♀️🤽🤽‍♂️🤾‍♀️🤾🤾‍♂️🌊🚣‍♀️🚣🚣‍♂️🧗‍♀️🧗🧗‍♂️🚵‍♀️🚵🚵‍♂️🚴‍♀️🚴🚴‍♂️🏆🥇🥈🥉🏅🎖️🏵️🎗️🎫🎟️🎪🤹‍♀️🤹🤹‍♂️🎭🩰🎨🎬🎤🎧🎼🎹🥁🪘🎷🎺🪗🎸🪕🎻🎲♟️🎯🎳🎮🎰🧩";
        let (_result, count) = remove_emojis(input);
        assert!(count > 50);
    }

    #[test]
    fn test_objects() {
        let input = "Objects: 👓🕶️🥽🥼🦺👔👕👖🧣🧤🧥🧦👗👘🥻🩱🩲🩳👙👚👛👜👝🛍️🎒🩴👞👟🥾🥿👠👡🩰👢👑👒🎩🎓🧢🪖⛑️📿💄💍💎🔇🔈🔉🔊📢📣📯🔔🔕🎼🎵🎶🎙️🎚️🎛️🎤🎧📻🎷🎸🎹🎺🎻🪕🥁🪘📱📲☎️📞📟📠🔋🔌💻🖥️🖨️⌨️🖱️🖲️💽💾💿📀🧮🎥🎞️📽️🎬📺📷📸📹📼🔍🔎🕯️💡🔦🏮🪔📔📕📖📗📘📙📚📓📒📃📜📄📰🗞️📑🔖🏷️💰🪙💴💵💶💷💸💳🧾💹✉️📧📨📩📤📥📦📫📪📬📭📮🗳️✏️✒️🖋️🖊️🖌️🖍️📝💼📁📂🗂️📅📆🗒️🗓️📇📈📉📊📋📌📍📎🖇️📏📐✂️🗃️🗄️🗑️🔒🔓🔏🔐🔑🗝️🔨🪓⛏️⚒️🛠️🗡️⚔️🔫🪃🏹🛡️🪚🔧🪛🔩⚙️🗜️⚖️🦯🔗⛓️🪝🧰🧲🪜⚗️🧪🧫🧬🔬🔭📡💉🩸💊🩹🩺🌡️🚽🚰🚿🛁🛀🧴🧵🧶🪡🧷🎽🥽🥼🦺";
        let (_result, count) = remove_emojis(input);
        assert!(count > 50);
    }

    #[test]
    fn test_newline_and_whitespace_preserved() {
        let input = "Line 1 😀\nLine 2 🌍\n\nLine 4 🔥";
        let (result, count) = remove_emojis(input);
        assert_eq!(result, "Line 1 \nLine 2 \n\nLine 4 ");
        assert_eq!(count, 3);
    }

    #[test]
    fn test_copyright_and_trademark() {
        let input = "Legal: © ® ™";
        let (result, count) = remove_emojis(input);
        assert_eq!(count, 3);
        assert_eq!(result, "Legal:   ");
    }

    #[test]
    fn test_is_emoji_individual() {
        assert!(is_emoji('😀'));
        assert!(is_emoji('🚀'));
        assert!(is_emoji('🌍'));
        assert!(!is_emoji('a'));
        assert!(!is_emoji('A'));
        assert!(!is_emoji('1'));
        assert!(!is_emoji('é'));
        assert!(!is_emoji('日'));
    }

    #[test]
    fn test_process_file_with_temp_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "Hello 😀 World 🌍!").unwrap();
        let path = temp_file.path().to_str().unwrap();

        let args = Args {
            files: vec![path.to_string()],
            backup: false,
            inplace: true,
            dry_run: false,
        };

        let result = process_file(path, &args);
        assert!(result.success);
        assert_eq!(result.emojis_found, 2);

        let content = fs::read_to_string(path).unwrap();
        assert_eq!(content.trim(), "Hello  World !");
    }

    #[test]
    fn test_process_file_dry_run() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "Test 🚀 content").unwrap();
        let path = temp_file.path().to_str().unwrap();

        let args = Args {
            files: vec![path.to_string()],
            backup: false,
            inplace: false,
            dry_run: true,
        };

        let result = process_file(path, &args);
        assert!(result.success);
        assert_eq!(result.emojis_found, 1);

        let content = fs::read_to_string(path).unwrap();
        assert!(content.contains("🚀"));
    }

    #[test]
    fn test_process_file_backup() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "Backup test 🔥").unwrap();
        let path = temp_file.path().to_str().unwrap();
        let backup_path = format!("{}.bak", path);

        let args = Args {
            files: vec![path.to_string()],
            backup: true,
            inplace: false,
            dry_run: false,
        };

        let result = process_file(path, &args);
        assert!(result.success);

        assert!(fs::metadata(&backup_path).is_ok());
        let backup_content = fs::read_to_string(&backup_path).unwrap();
        assert!(backup_content.contains("🔥"));

        fs::remove_file(&backup_path).ok();
    }

    #[test]
    fn test_process_file_nonexistent() {
        let args = Args {
            files: vec!["nonexistent_file.txt".to_string()],
            backup: false,
            inplace: false,
            dry_run: false,
        };

        let result = process_file("nonexistent_file.txt", &args);
        assert!(!result.success);
        assert!(result.error.is_some());
    }

    #[test]
    fn test_read_write_functions() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "Test content").unwrap();
        let path = temp_file.path();

        let content = read_input(path).unwrap();
        assert!(content.contains("Test content"));

        write_output(path, "New content").unwrap();
        let new_content = fs::read_to_string(path).unwrap();
        assert_eq!(new_content, "New content");
    }

    #[test]
    fn test_print_report_empty() {
        let results: Vec<ProcessResult> = vec![];
        print_report(&results);
    }

    #[test]
    fn test_print_report_with_results() {
        let results = vec![
            ProcessResult {
                file: "test1.txt".to_string(),
                emojis_found: 5,
                success: true,
                error: None,
            },
            ProcessResult {
                file: "test2.txt".to_string(),
                emojis_found: 0,
                success: false,
                error: Some("File not found".to_string()),
            },
        ];
        print_report(&results);
    }

    #[test]
    fn test_cli_args_parsing() {
        let args = Args::parse_from(["nomoji", "file1.txt", "file2.txt"]);
        assert_eq!(args.files.len(), 2);
        assert!(!args.backup);
        assert!(!args.inplace);
        assert!(!args.dry_run);

        let args = Args::parse_from(["nomoji", "-b", "-i", "file.txt"]);
        assert!(args.backup);
        assert!(args.inplace);

        let args = Args::parse_from(["nomoji", "--dry-run", "file.txt"]);
        assert!(args.dry_run);
    }

    #[test]
    fn test_zero_width_joiner() {
        let input = "Family: 👨‍👩‍👧‍👦";
        let (result, count) = remove_emojis(input);
        assert!(count >= 4);
        assert!(!result.contains('👨'));
        assert!(!result.contains('👩'));
        assert!(!result.contains('👧'));
        assert!(!result.contains('👦'));
    }

    #[test]
    fn test_complex_emoji_sequence() {
        let input = "Couple: 👩‍❤️‍👨 Profession: 👨‍🚀👩‍⚕️";
        let (result, count) = remove_emojis(input);
        assert!(count >= 6);
        assert!(!result.contains("👨‍🚀"));
        assert!(!result.contains("👩‍⚕️"));
    }

    #[test]
    fn test_dingbats_and_miscellaneous() {
        let input = "Dingbats: ✀✁✂✃✄✅✆✇✈✉✊✋✌✍✎✏✐✑✒✓✔✕✖✗✘✙✚✛✜✝✞✟✠✡✢✣✣✥✦✧✨✩✪✫✬✭✮✯✰✱✲✳✴✵✶✷✸✹✺✻✼✽✾✿❀❁❂❃❄❅❆❇❈❉❊❋❌❍❎❏❐❑❒❓❔❕❖❗❘❙❚❛❜❝❞❟❠❡❢❣❤❥❦❧❨❩❪❫❬❭❮❯❰❱❲❳❴❵❶❷❸❹❺❻❼❽❾❿➀➁➂➃➄➅➆➇➈➉➊➋➌➍➎➏➐➑➒➓➔➕➖➗➘➙➚➛➜➝➞➟➠➡➢➣➤➥➦➧➨➩➪➫➬➭➮➯➰➱➲➳➴➵➶➷➸➹➺➻➼➽➾➿";
        let (_result, count) = remove_emojis(input);
        assert!(count > 50);
    }

    #[test]
    fn test_transport_symbols() {
        let input = "Transport: 🚀🛸🚁🚂🚃🚄🚅🚆🚇🚈🚉🚊🚋🚌🚍🚎🚏🚐🚑🚒🚓🚔🚕🚖🚗🚘🚙🚚🚛🚜🚝🚞🚟🚠🚡🚢🚣🚤🚥🚦🚧🚨🚩🚪🚫🚬🚭🚮🚯🚰🚱🚲🚳🚴🚵🚶🚷🚸🚹🚺🚻🚼🚽🚾🚿🛀🛁🛂🛃🛄🛅🛆🛇🛈🛉🛊🛋🛌🛍🛎🛏🛐🛑🛒🛓🛔🛕🛖🛗🛘🛙🛚🛛🛜🛝🛞🛟🛠🛡🛢🛣🛤🛥🛦🛧🛨🛩🛪🛫🛬🛭🛮🛯🛰🛱🛲🛳🛴🛵🛶🛷🛸🛹🛺🛻🛼🛽🛾🛿";
        let (_result, count) = remove_emojis(input);
        assert!(count > 50);
    }

    #[test]
    fn test_large_file_simulation() {
        let mut large_input = String::with_capacity(10000);
        for i in 0..1000 {
            large_input.push_str(&format!("Line {} with emoji 😀 and text 🚀 ", i));
        }

        let (result, count) = remove_emojis(&large_input);
        assert_eq!(count, 2000);
        assert!(!result.contains("😀"));
        assert!(!result.contains("🚀"));
        assert!(result.contains("Line 0"));
        assert!(result.contains("Line 999"));
    }

    #[test]
    fn test_special_unicode_control_chars() {
        let input = "Text with \u{0000}\u{0001}\u{0002} and emoji 😀";
        let (result, count) = remove_emojis(input);
        assert_eq!(count, 1);
        assert!(result.contains("\u{0000}"));
        assert!(!result.contains("😀"));
    }

    #[test]
    fn test_mixed_scripts_with_emoji() {
        let input = "English: Hello 😀 | 日本語: こんにちは 🎌 | العربية: مرحبا 🕌 | עברית: שלום ✡️ | 中文: 你好 🇨🇳";
        let (result, count) = remove_emojis(input);
        assert!(count >= 5);
        assert!(result.contains("English:"));
        assert!(result.contains("日本語:"));
        assert!(result.contains("العربية:"));
        assert!(result.contains("עברית:"));
        assert!(result.contains("中文:"));
    }
}
