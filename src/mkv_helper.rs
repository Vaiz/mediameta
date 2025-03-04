use crate::{MetaData, Result};
use matroska::Settings;
use std::io;
use std::time::SystemTime;

/// Extracts metadata from an MKV (Matroska) container.
///
/// This function uses the `Matroska` crate to retrieve metadata, such as video dimensions and creation date,
/// from an MKV container.
pub fn extract_mkv_metadata<R: io::Read + io::Seek>(io: R) -> Result<MetaData> {
    let matroska = matroska::Matroska::open(io)?;
    let video_track = matroska.video_tracks().next();
    let (width, height) = if let Some(video_track) = video_track {
        if let Settings::Video(video) = &video_track.settings {
            (video.pixel_width, video.pixel_height)
        } else {
            (0, 0)
        }
    } else {
        (0, 0)
    };

    let creation_date = matroska
        .info
        .date_utc
        .as_ref()
        .map(convert_mkv_time_to_system_time);

    Ok(MetaData {
        width,
        height,
        creation_date,
    })
}

pub(crate) fn extract_mkv_creation_date<R: io::Read + io::Seek>(io: R) -> Result<SystemTime> {
    let matroska = matroska::Matroska::open(io)?;

    matroska
        .info
        .date_utc
        .as_ref()
        .map(convert_mkv_time_to_system_time)
        .ok_or(crate::Error::CreationDateNotFound)
}

fn convert_mkv_time_to_system_time(mkv_time: &matroska::DateTime) -> SystemTime {
    use chrono::{Duration, TimeZone, Utc};
    // MKV creation time is based on seconds since 2001-01-01
    let mkv_time: i64 = mkv_time.clone().into();
    let utc = Utc.with_ymd_and_hms(2001, 1, 1, 0, 0, 0).unwrap() + Duration::nanoseconds(mkv_time);
    utc.into()
}
