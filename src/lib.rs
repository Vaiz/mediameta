mod mp4_helper;

//use matroska::MatroskaReader;
use anyhow::Context;
use std::io::{Read, Seek};
use std::path::Path;
use std::time::SystemTime;

#[derive(Debug)]
pub struct MetaData {
    width: u16,
    height: u16,
    creation_date: SystemTime,
}

pub fn extract_metadata(file_path: &str) -> anyhow::Result<MetaData> {
    let path = Path::new(file_path);
    let file_extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();

    match file_extension.as_str() {
        "mp4" => mp4_helper::extract_mp4_metadata(file_path),
        "mkv" => extract_mkv_metadata(file_path),
        _ => anyhow::bail!("Unsupported file format: {}", file_extension)
    }
}

fn extract_mkv_metadata(file_path: &str) -> anyhow::Result<MetaData> {
    unimplemented!()
    /*
    let file = File::open(file_path).with_context(|| format!("Failed to open file: {}", file_path))?;
    let reader = BufReader::new(file);
    let matroska = MatroskaReader::new(reader).with_context(|| "Failed to read MKV file")?;

    // Extract width and height from the video track
    let track = matroska.tracks().iter().find(|t| t.is_video());
    let (width, height) = if let Some(video_track) = track {
        let width = video_track.video().map(|v| v.pixel_width).unwrap_or(0);
        let height = video_track.video().map(|v| v.pixel_height).unwrap_or(0);
        (width, height)
    } else {
        (0, 0) // default values if no video track is found
    };

    // Retrieve creation date from MKV metadata (Matroska files may have a date field in the Segment Info)
    let creation_date = matroska.segment().info().and_then(|info| {
        info.date_utc().map(|date| {
            // Convert from nanoseconds to `SystemTime`
            UNIX_EPOCH + Duration::from_nanos(date as u64)
        })
    });

    Ok(MetaData {
        width,
        height,
        size,
        creation_date,
    })

     */
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> anyhow::Result<()> {
        let file_path = "D:/tmp/1.mp4"; // Replace with your file path
        println!("{:?}", extract_metadata(file_path)?);
        Ok(())
    }
}
