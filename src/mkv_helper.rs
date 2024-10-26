use crate::MetaData;
use anyhow::Context;
use matroska::Settings;
use std::io;
use std::time::{Duration, SystemTime};

pub(crate) fn extract_mkv_metadata<R: io::Read + io::Seek>(io: R) -> anyhow::Result<MetaData> {
    let matroska = matroska::Matroska::open(io).with_context(|| "Failed to load Matroska container")?;
    let video_track = matroska.video_tracks().next();
    let (width, height) = if let Some(video_track) = video_track {
        if let Settings::Video(video) = &video_track.settings {
            (video.display_width.unwrap_or(0), video.display_height.unwrap_or(0))
        } else {
            (0, 0)
        }
    } else {
        (0, 0)
    };

    let creation_date = matroska.info.date_utc.as_ref().map(convert_mkv_time_to_system_time);

    Ok(MetaData {
        width,
        height,
        creation_date,
    })
}

fn convert_mkv_time_to_system_time(mkv_time: &matroska::DateTime) -> SystemTime {
    // MKV creation time is based on seconds since 2001-01-01
    let mkv_epoch = std::time::UNIX_EPOCH + Duration::from_secs(978_307_200);
    let mkv_time: i64 = mkv_time.clone().into();
    mkv_epoch + Duration::from_secs(mkv_time as u64)
}