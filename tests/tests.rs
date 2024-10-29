#[path = "tests/exif.rs"]
mod exif;
#[path = "tests/metadata.rs"]
mod metadata;
#[path = "tests/mkv.rs"]
mod mkv;
#[path = "tests/mp4.rs"]
mod mp4;

#[cfg(feature = "mediainfo")]
#[path = "tests/mediainfo.rs"]
mod mediainfo;

#[path = "tests/extract_file_creation_date.rs"]
mod extract_file_creation_date;

use mediameta::parse_date;
