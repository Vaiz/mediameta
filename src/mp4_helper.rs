use crate::MetaData;
use anyhow::Context;
use mp4::Mp4Track;
use std::collections::HashMap;
use std::io;
use std::time::SystemTime;

/// Extracts metadata from an MP4 container.
///
/// This function utilizes the `mp4` crate to obtain metadata, including dimensions and creation
/// date, from an MP4 container.
pub fn extract_mp4_metadata<R: io::Read + io::Seek>(
    io: R,
    file_size: u64,
) -> anyhow::Result<MetaData> {
    let mp4 =
        mp4::Mp4Reader::read_header(io, file_size).with_context(|| "Failed to read MP4 header")?;

    let video_track = find_video_track(mp4.tracks());
    let (width, height) = if let Some(track) = video_track {
        (track.width() as u64, track.height() as u64)
    } else {
        (0, 0)
    };

    let creation_date = convert_mp4_time_to_system_time(mp4.moov.mvhd.creation_time);

    Ok(MetaData {
        width,
        height,
        creation_date,
    })
}

fn convert_mp4_time_to_system_time(mp4_time: u64) -> Option<SystemTime> {
    use chrono::{Duration, TimeZone, Utc};

    if mp4_time == 0 {
        return None;
    }
    // MP4 creation time is based on seconds since 1904-01-01
    let utc =
        Utc.with_ymd_and_hms(1904, 1, 1, 0, 0, 0).unwrap() + Duration::seconds(mp4_time as i64);
    Some(utc.into())
}

fn find_video_track(tracks: &HashMap<u32, Mp4Track>) -> Option<&Mp4Track> {
    const NOT_VIDEO: mp4::TrackType = mp4::TrackType::Subtitle;
    tracks
        .iter()
        .find(|(_, track)| mp4::TrackType::Video == track.track_type().unwrap_or(NOT_VIDEO))
        .map(|(_, track)| track)
}
