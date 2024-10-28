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

use mediameta::parse_date;
