use crate::MetaData;
use anyhow::Context;
use mp4::Mp4Track;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::time::{Duration, SystemTime};

pub(crate) fn extract_mp4_metadata(file_path: &str) -> anyhow::Result<MetaData> {
    let file = File::open(file_path).with_context(|| format!("Failed to open file: {}", file_path))?;
    let size = file.metadata()?.len();
    let reader = BufReader::new(file);
    let mp4 = mp4::Mp4Reader::read_header(reader, size)
        .with_context(|| "Failed to read MP4 header")?;

    let video_track = find_video_track(mp4.tracks());
    let (width, height) = if let Some(track) = video_track {
        (track.width(), track.height())
    } else {
        (0, 0)
    };

    let creation_date = {
        // MP4 creation time is based on seconds since 1904-01-01
        let mp4_epoch: SystemTime = std::time::UNIX_EPOCH - Duration::from_secs(2_082_844_800);
        mp4_epoch + Duration::from_secs(mp4.moov.mvhd.creation_time)
    };

    Ok(MetaData {
        width,
        height,
        creation_date,
    })
}

fn find_video_track(tracks: &HashMap<u32, Mp4Track>) -> Option<&Mp4Track> {
    const NOT_VIDEO: mp4::TrackType = mp4::TrackType::Subtitle;
    tracks
        .iter()
        .find(|(_, track)|
            mp4::TrackType::Video == track.track_type().unwrap_or(NOT_VIDEO))
        .map(|(_, track)| track)
}