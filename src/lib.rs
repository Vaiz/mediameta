//! This library provides a straightforward API to extract essential metadata (such as dimensions
//! and creation date) from media files. It operates efficiently with multiple file types and
//! includes an optional fallback using the mediainfo tool for extended metadata extraction.

#![cfg_attr(docsrs, feature(doc_cfg))]

mod exif_helper;
mod mkv_helper;
mod mp4_helper;

#[cfg(feature = "mediainfo")]
#[cfg_attr(docsrs, doc(cfg(feature = "mediainfo")))]
pub mod mediainfo;

use anyhow::Context;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::path::Path;
use std::time::SystemTime;

pub use exif_helper::extract_exif_metadata;
pub use mkv_helper::extract_mkv_metadata;
pub use mp4_helper::extract_mp4_metadata;

/// Represents the extracted metadata for a media file.
#[derive(Debug, PartialEq)]
pub struct MetaData {
    pub width: u64,
    pub height: u64,
    pub creation_date: Option<SystemTime>,
}

impl Display for MetaData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let creation_date = match self.creation_date {
            Some(time) => {
                let datetime: chrono::DateTime<chrono::Utc> = time.into();
                datetime.to_rfc3339()
            }
            None => "None".to_string(),
        };
        write!(
            f,
            "width: {}, height: {}, creation_date: {}",
            self.width, self.height, creation_date
        )
    }
}

/// Enum representing supported media container types.
///
/// This enum defines the container types that can be processed by the library. The `Exif` variant
/// accepts a custom string to store file extensions for future use, enabling additional flexibility
/// for Exif-based media.
#[derive(Debug, PartialEq)]
pub enum ContainerType {
    Mp4,
    Mkv,
    Exif(String),
}

/// Detects the container type of a media file based on its extension.
///
/// This function determines the container type from file extension, which is required by the
/// [`extract_metadata`] function. It can identify common types, including MP4, MKV, and Exif-based
/// formats.
pub fn get_container_type<P: AsRef<Path>>(file_path: P) -> anyhow::Result<ContainerType> {
    let file_extension = file_path
        .as_ref()
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();

    match file_extension.as_str() {
        "mp4" => Ok(ContainerType::Mp4),
        "mkv" => Ok(ContainerType::Mkv),
        "jpg" | "jpeg" | "tiff" | "tif" | "webp" | "heif" | "heic" | "dng" | "cr2" | "cr3"
        | "nef" | "arw" | "raf" | "rw2" | "orf" => Ok(ContainerType::Exif(file_extension)),
        _ => anyhow::bail!("Unsupported container format: {}", file_extension),
    }
}

/// Combines two methods of metadata extraction to ensure a comprehensive result.
///
/// This function requires `metainfo` feature to be enabled. It attempts to retrieve metadata using
/// [`extract_file_metadata`]. If unsuccessful, it falls back to using the mediainfo tool. This is
/// the most efficient way of receiving metadata of any media file.
#[cfg(feature = "mediainfo")]
#[cfg_attr(docsrs, doc(cfg(feature = "mediainfo")))]
pub fn extract_combined_metadata<P: AsRef<Path>>(file_path: P) -> anyhow::Result<MetaData> {
    let result1 = crate::extract_file_metadata(&file_path);
    if let Ok(meta) = &result1 {
        if meta.height > 0 && meta.width > 0 && meta.creation_date.is_some() {
            return result1;
        }
    }
    let result2 = crate::mediainfo::extract_metadata(&file_path);
    if result1.is_err() {
        return result2;
    }
    if result2.is_err() {
        return result1;
    }

    let meta1 = result1?;
    let meta2 = result2?;
    Ok(MetaData {
        width: if meta1.width > 0 {
            meta1.width
        } else {
            meta2.width
        },
        height: if meta1.height > 0 {
            meta1.height
        } else {
            meta2.height
        },
        creation_date: if meta1.creation_date.is_some() {
            meta1.creation_date
        } else {
            meta2.creation_date
        },
    })
}

/// Extracts metadata from a file.
///
/// This function opens a file using [BufReader], and then calls
/// [`extract_metadata`].
pub fn extract_file_metadata<P: AsRef<Path>>(file_path: P) -> anyhow::Result<MetaData> {
    let container_type = get_container_type(&file_path)?;
    let file = File::open(&file_path).with_context(|| {
        format!(
            "Failed to open file {}",
            file_path.as_ref().to_string_lossy()
        )
    })?;
    let size = file.metadata()?.len();
    let reader = BufReader::new(file);
    extract_metadata(reader, size, container_type)
}

/// Extracts metadata from a buffered stream.
///
/// This function is useful when user code has already opened the file. It avoids unnecessary
/// additional file access, making metadata extraction more efficient.
pub fn extract_metadata<R>(
    io: R,
    file_size: u64,
    container_type: ContainerType,
) -> anyhow::Result<MetaData>
where
    R: io::BufRead + io::Seek,
{
    match container_type {
        ContainerType::Mp4 => extract_mp4_metadata(io, file_size),
        ContainerType::Mkv => extract_mkv_metadata(io),
        ContainerType::Exif(extension) => extract_exif_metadata(io, extension),
    }
}

/// Extracts the creation date from a media file.
///
/// This function attempts to retrieve the creation date of a media file using Rust's native
/// libraries for optimal performance. If this extraction fails and the `mediainfo` feature is
/// enabled, it falls back to the external `mediainfo` tool to retrieve the date.
///
/// Since it only extracts the creation date, this function is more efficient than
/// [`extract_combined_metadata`], which gathers additional metadata fields.
pub fn extract_file_creation_date<P: AsRef<Path>>(file_path: P) -> anyhow::Result<SystemTime> {
    let container_type = get_container_type(&file_path)?;
    let file = File::open(&file_path).with_context(|| {
        format!(
            "Failed to open file {}",
            file_path.as_ref().to_string_lossy()
        )
    })?;
    let file_size = file.metadata()?.len();
    let io = BufReader::new(file);
    let creation_date = match container_type {
        ContainerType::Mp4 => mp4_helper::extract_mp4_creation_date(io, file_size),
        ContainerType::Mkv => mkv_helper::extract_mkv_creation_date(io),
        ContainerType::Exif(_) => exif_helper::extract_exif_creation_date(io),
    };

    #[cfg(feature = "mediainfo")]
    if creation_date.is_err() {
        if let Ok(meta) = mediainfo::extract_metadata(file_path) {
            if let Some(creation_date) = meta.creation_date {
                return Ok(creation_date);
            }
        }
    }

    creation_date
}

/// This function is solely for test purposes
#[doc(hidden)]
pub fn parse_date(date: &str) -> SystemTime {
    use chrono::prelude::*;
    let naive_datetime =
        NaiveDateTime::parse_from_str(date, "%Y-%m-%dT%H:%M:%S").expect("Failed to parse date");

    naive_datetime.and_utc().into()
}
