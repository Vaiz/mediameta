use crate::MetaData;
use anyhow::Context;
use exif::Tag;
use std::io;

use chrono::prelude::*;
use std::time::SystemTime;

/// Extracts metadata from an Exif-based media file.
///
/// This function reads Exif metadata from an image or media file using the `kamadak-exif` crate.
/// If resolution information is missing from the Exif metadata and the `image` feature is enabled,
/// it attempts to resolve the image's resolution using the `image` crate.
pub fn extract_exif_metadata<R>(mut io: R, extension: String) -> anyhow::Result<MetaData>
where
    R: io::BufRead + io::Seek,
{
    let exifreader = exif::Reader::new();
    let exif = exifreader
        .read_from_container(&mut io)
        .with_context(|| "Failed to read Exif container")?;

    let (width, height) = get_width_and_height(&exif, io, extension);
    let creation_date = get_creation_date(&exif);

    Ok(MetaData {
        width,
        height,
        creation_date,
    })
}

pub(crate) fn extract_exif_creation_date<R>(mut io: R) -> anyhow::Result<SystemTime>
where
    R: io::BufRead + io::Seek,
{
    let exifreader = exif::Reader::new();
    let exif = exifreader
        .read_from_container(&mut io)
        .with_context(|| "Failed to read Exif container")?;

    get_creation_date(&exif).with_context(|| "EXIF doesn't contain creation date")
}

fn get_creation_date(exif: &exif::Exif) -> Option<SystemTime> {
    let creation_date = exif.get_field(Tag::DateTimeOriginal, exif::In::PRIMARY);
    if let Some(creation_date) = creation_date {
        let date_str = creation_date.display_value().with_unit(exif).to_string();
        let date = NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M:%S")
            .map(|naive_datetime| SystemTime::from(Utc.from_utc_datetime(&naive_datetime)))
            .with_context(|| format!("Failed to parse datetime {date_str}")).unwrap();
        Some(date)
    } else {
        None
    }
}

#[allow(unused_mut, unused_variables)]
fn get_width_and_height<R>(exif: &exif::Exif, mut io: R, extension: String) -> (u64, u64)
where
    R: io::BufRead + io::Seek,
{
    let width = exif
        .get_field(Tag::PixelXDimension, exif::In::PRIMARY)
        .and_then(|field| field.value.get_uint(0))
        .unwrap_or(0) as u64;

    let height = exif
        .get_field(Tag::PixelYDimension, exif::In::PRIMARY)
        .and_then(|field| field.value.get_uint(0))
        .unwrap_or(0) as u64;

    #[cfg(feature = "image")]
    if width == 0 || height == 0 {
        if let Some(format) = image::ImageFormat::from_extension(extension) {
            let _ = io.seek(std::io::SeekFrom::Start(0));
            let img = image::ImageReader::with_format(io, format);
            if let Ok((width, height)) = img.into_dimensions() {
                return (width as u64, height as u64);
            }
        }
    }

    (width, height)
}
