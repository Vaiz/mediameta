use std::env;

/// cargo run --example file_info test-data/sample-mkv-files-sample_640x360_with_date.mkv
fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <file_path>", args[0]);
        return Ok(());
    }

    let file_path = &args[1];
    let meta = mediameta::extract_file_metadata(file_path)?;
    println!("{meta}");
    Ok(())
}
