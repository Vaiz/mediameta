use anyhow::Context;
use std::env;
use std::fs;
use std::time::SystemTime;

/// cargo run --features=mediainfo --example mediainfo_ls test-data
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
                    let meta = video_info::mediainfo::extract_metadata(entry.path());
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

fn format_creation_date(creation_date: Option<SystemTime>) -> String {
    match creation_date {
        Some(time) => {
            let datetime: chrono::DateTime<chrono::Utc> = time.into();
            datetime.to_rfc3339()
        }
        None => "None".to_string(),
    }
}
