mod exif_helper;
mod mkv_helper;
mod mp4_helper;

#[cfg(feature = "mediainfo")]
mod mediainfo_helper;

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

#[cfg(feature = "mediainfo")]
pub mod mediainfo {
    pub use super::mediainfo_helper::extract_metadata;
}

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

#[derive(Debug, PartialEq)]
pub enum ContainerType {
    Mp4,
    Mkv,
    Exif(String),
}

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

/// This function is solely for test purposes
#[doc(hidden)]
pub fn parse_date(date: &str) -> SystemTime {
    use chrono::prelude::*;
    let naive_datetime =
        NaiveDateTime::parse_from_str(date, "%Y-%m-%dT%H:%M:%S").expect("Failed to parse date");

    naive_datetime.and_utc().into()
}
