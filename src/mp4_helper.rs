use crate::Result;
use crate::{Error, MetaData};
use mp4::Track;
use std::io;
use std::time::SystemTime;

/// Extracts metadata from an MP4 container.
///
/// This function utilizes the `mp4` crate to obtain metadata, including dimensions and creation
/// date, from an MP4 container.
pub fn extract_mp4_metadata<R: io::Read + io::Seek>(io: R, file_size: u64) -> Result<MetaData> {
    let mp4 = mp4::Mp4::read(io, file_size)?;

    let video_track = find_video_track(mp4.tracks());
    let (width, height) = if let Some(track) = video_track {
        (track.width as u64, track.height as u64)
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

pub(crate) fn extract_mp4_creation_date<R: io::Read + io::Seek>(
    io: R,
    file_size: u64,
) -> Result<SystemTime> {
    let mp4 = mp4::Mp4::read(io, file_size)?;

    convert_mp4_time_to_system_time(mp4.moov.mvhd.creation_time).ok_or(Error::CreationDateNotFound)
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

fn find_video_track<'a>(
    tracks: impl IntoIterator<Item = (&'a u32, &'a Track)>,
) -> Option<&'a Track> {
    use mp4::TrackKind;
    const NOT_VIDEO: TrackKind = TrackKind::Subtitle;
    tracks
        .into_iter()
        .find(|(_, track)| TrackKind::Video == track.kind.unwrap_or(NOT_VIDEO))
        .map(|(_, track)| track)
}
