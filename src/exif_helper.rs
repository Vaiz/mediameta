use crate::MetaData;
use anyhow::Context;
use exif::Tag;
use std::io;

use chrono::prelude::*;
use std::time::SystemTime;

pub(crate) fn extract_exif_metadata<R>(mut io: R) -> anyhow::Result<MetaData>
where
    R: io::BufRead + io::Seek,
{
    let exifreader = exif::Reader::new();
    let exif = exifreader
        .read_from_container(&mut io)
        .with_context(|| "Failed to read Exif container")?;

    let width = exif
        .get_field(Tag::PixelXDimension, exif::In::PRIMARY)
        .and_then(|field| field.value.get_uint(0))
        .unwrap_or(0) as u64;

    let height = exif
        .get_field(Tag::PixelYDimension, exif::In::PRIMARY)
        .and_then(|field| field.value.get_uint(0))
        .unwrap_or(0) as u64;

    let creation_date = exif.get_field(Tag::DateTime, exif::In::PRIMARY);
    let creation_date = if let Some(creation_date) = creation_date {
        let date_str = creation_date.display_value().with_unit(&exif).to_string();
        let date = NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M:%S")
            .map(|naive_datetime| SystemTime::from(Utc.from_utc_datetime(&naive_datetime)))
            .with_context(|| format!("Failed to parse datetime {date_str}"))?;
        Some(date)
    } else {
        None
    };

    Ok(MetaData {
        width,
        height,
        creation_date,
    })
}
