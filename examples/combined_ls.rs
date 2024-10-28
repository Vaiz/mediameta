use anyhow::Context;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;
use video_info::MetaData;

/// cargo run --features=mediainfo --example combined_ls test-data
fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <directory_path>", args[0]);
        return Ok(());
    }

    let dir_path = &args[1];

    let entries =
        fs::read_dir(dir_path).with_context(|| format!("failed to read_dir {dir_path}"))?;
    println!("{:<78} Resolution  Creation date", "Name");
    for entry in entries {
        match entry {
            Ok(entry) => {
                let name = entry.file_name();
                if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
                    let meta = extract_metadata(entry.path());
                    if let Ok(meta) = meta {
                        println!(
                            "{:<80} {:>4}x{:<4} {}",
                            name.to_string_lossy(),
                            meta.width,
                            meta.height,
                            format_creation_date(meta.creation_date)
                        );
                    } else {
                        println!("{}", name.to_string_lossy());
                    }
                } else {
                    println!("[{}]", name.to_string_lossy());
                }
            }
            Err(err) => eprintln!("Error reading entry: {}", err),
        }
    }

    Ok(())
}

fn extract_metadata(path: PathBuf) -> anyhow::Result<MetaData> {
    let result1 = video_info::extract_file_metadata(&path);
    if let Ok(meta) = &result1 {
        if meta.height > 0 && meta.width > 0 && meta.creation_date.is_some() {
            return result1;
        }
    }
    let result2 = video_info::mediainfo::extract_metadata(&path);
    if result1.is_err() {
        return result2;
    }
    if result2.is_err() {
        return result1;
    }

    let meta1 = result1?;
    let meta2 = result2?;
    Ok(MetaData {
        width: if meta1.width > 0 {
            meta1.width
        } else {
            meta2.width
        },
        height: if meta1.height > 0 {
            meta1.height
        } else {
            meta2.height
        },
        creation_date: if meta1.creation_date.is_some() {
            meta1.creation_date
        } else {
            meta2.creation_date
        },
    })
}

fn format_creation_date(creation_date: Option<SystemTime>) -> String {
    match creation_date {
        Some(time) => {
            let datetime: chrono::DateTime<chrono::Utc> = time.into();
            datetime.to_rfc3339()
        }
        None => "None".to_string(),
    }
}
