#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Unsupported container type: {0}")]
    UnsupportedContainerType(String),

    #[error("Creation date not found")]
    CreationDateNotFound,

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Failed to read file exif data: {0}")]
    ExifError(#[from] exif::Error),

    #[error("Failed to read matroska data: {0}")]
    MatroskaError(#[from] matroska::Error),

    #[error("Failed to read mp4 data: {0}")]
    Mp4Error(#[from] mp4::Error),

    #[error("Failed to parse datetime: {0}")]
    FailedToParseDateTime(String),

    #[cfg(feature = "mediainfo")]
    #[error("Failed to run mediainfo command: {0}")]
    MediainfoError(String),

    #[cfg(feature = "mediainfo")]
    #[error("Failed to find mediainfo binary. More information: https://github.com/Vaiz/mediameta/blob/master/mediainfo.md")]
    MediaInfoToolNotFound,

    #[cfg(feature = "mediainfo")]
    #[error("Failed to find any useful metadata in the file")]
    MetadataNotFound,

    #[cfg(feature = "mediainfo")]
    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
