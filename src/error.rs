#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Unsupported container type: {0}")]
    UnsupportedContainerType(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Exif error: {0}")]
    Exif(#[from] exif::Error),

    #[error("Matroska error: {0}")]
    Matroska(#[from] matroska::Error),

    #[error("MP4 error: {0}")]
    Mp4(#[from] mp4::Error),

    #[error("Creation date not found")]
    CreationDateNotFound,

    #[error("Failed to parse datetime: {0}")]
    DateTimeParseError(String),

    #[cfg(feature = "mediainfo")]
    #[error("Mediainfo error: {0}")]
    Mediainfo(#[from] MediainfoError),
}

#[cfg(feature = "mediainfo")]
#[derive(thiserror::Error, Debug)]
pub enum MediainfoError {
    #[error("Failed to find mediainfo binary. More information: https://github.com/Vaiz/mediameta/blob/master/mediainfo.md")]
    ToolNotFound,

    #[error("Failed to run mediainfo command: {0}")]
    CommandError(String),

    #[error("Failed to find any useful metadata in the file")]
    MetadataNotFound,

    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
