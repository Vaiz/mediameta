use crate::error::MediainfoError;
use crate::{Error, MetaData, Result};
use chrono::prelude::*;
use cmd_lib::run_fun;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;
use std::time::SystemTime;

static MEDIAINFO_PATH: LazyLock<Option<PathBuf>> = LazyLock::new(|| which::which("mediainfo").ok());

/// Extracts metadata from a media file using the `mediainfo` tool.
///
/// This function relies on the external `mediainfo` tool to gather metadata. It requires that
/// `mediainfo` is installed and available in the system's path.
///
/// ## Notes
/// This function can provide comprehensive metadata, but usually it is slower than
/// [`extract_file_metadata`](crate::extract_file_metadata).
pub fn extract_metadata<P: AsRef<Path>>(file_path: P) -> Result<MetaData> {
    let mediainfo = (*MEDIAINFO_PATH)
        .as_ref()
        .ok_or(MediainfoError::ToolNotFound)?;
    let file_path = file_path.as_ref();
    if !file_path.exists() {
        return Err(std::io::Error::from(std::io::ErrorKind::NotFound).into());
    }

    let result = run_fun!($mediainfo --Output=JSON $file_path)
        .map_err(|e| MediainfoError::CommandError(e.to_string()))?;
    extract_metadata_from_json(&result)
}

fn extract_metadata_from_json(json: &str) -> Result<MetaData> {
    let root: Root = serde_json::from_str(json).map_err(MediainfoError::from)?;
    let mut metadata = MetaData {
        width: 0,
        height: 0,
        creation_date: None,
    };
    let mut is_media = false;
    for track in root.media.track {
        match track {
            Track::General(general) => {
                if let Some(recorded_date) = general.recorded_date {
                    metadata.creation_date = Some(parse_datetime(&recorded_date)?);
                } else if let Some(encoded_date) = general.encoded_date {
                    metadata.creation_date = Some(parse_datetime(&encoded_date)?);
                }
            }
            Track::Video(video) => {
                metadata.width = video.width;
                metadata.height = video.height;
                is_media = true;
            }
            Track::Image(image) => {
                metadata.width = image.width;
                metadata.height = image.height;
                is_media = true;
            }
            Track::Other => {}
        }
    }

    if is_media {
        Ok(metadata)
    } else {
        Err(MediainfoError::MetadataNotFound.into())
    }
}

#[derive(serde::Deserialize, Debug)]
#[serde(tag = "@type")]
enum Track {
    General(GeneralTrack),
    Video(VideoTrack),
    Image(ImageTrack),
    #[serde(other)]
    Other,
}

#[derive(serde::Deserialize, Debug)]
struct GeneralTrack {
    #[serde(rename = "Recorded_Date")]
    recorded_date: Option<String>,
    #[serde(rename = "Encoded_Date")]
    encoded_date: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
struct VideoTrack {
    #[serde(rename = "Width", deserialize_with = "parse_string_to_u64")]
    width: u64,
    #[serde(rename = "Height", deserialize_with = "parse_string_to_u64")]
    height: u64,
}

#[derive(serde::Deserialize, Debug)]
struct ImageTrack {
    #[serde(rename = "Width", deserialize_with = "parse_string_to_u64")]
    width: u64,
    #[serde(rename = "Height", deserialize_with = "parse_string_to_u64")]
    height: u64,
}

#[derive(serde::Deserialize, Debug)]
struct Media {
    track: Vec<Track>,
}

#[derive(serde::Deserialize, Debug)]
struct Root {
    media: Media,
}

fn parse_string_to_u64<'de, D>(deserializer: D) -> std::result::Result<u64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::{Error, Unexpected};
    let s: &str = serde::Deserialize::deserialize(deserializer)?;
    s.parse::<u64>()
        .map_err(|_| Error::invalid_value(Unexpected::Str(s), &"int"))
}

fn parse_datetime(datetime: &str) -> Result<SystemTime> {
    let datetime = datetime.trim_start_matches("UTC ").trim_end_matches(" UTC");
    let datetime: DateTime<Utc> = NaiveDateTime::parse_from_str(datetime, "%Y-%m-%d %H:%M:%S")
        .map_err(|_| Error::DateTimeParseError(datetime.to_string()))?
        .and_utc();
    Ok(datetime.into())
}

#[cfg(test)]
mod tests {
    use crate::MetaData;

    #[test]
    fn test_mp4() -> anyhow::Result<()> {
        let json_str = r#"
{
    "creatingLibrary": {
        "name": "MediaInfoLib",
        "version": "24.06",
        "url": "https://mediaarea.net/MediaInfo"
    },
    "media": {
        "@ref": "test-data\\sample-mp4-files-sample_640x360.mp4",
        "track": [
            {
                "@type": "General",
                "VideoCount": "1",
                "FileExtension": "mp4",
                "Format": "MPEG-4",
                "...": "..."
            },
            {
                "@type": "Video",
                "StreamOrder": "0",
                "ID": "1",
                "Format": "AVC",
                "Width": "640",
                "Height": "360",
                "...": "..."
            }
        ]
    }
}
"#;

        let metadata = super::extract_metadata_from_json(json_str)?;
        let expected = MetaData {
            width: 640,
            height: 360,
            creation_date: None,
        };
        assert_eq!(metadata, expected);
        Ok(())
    }

    #[test]
    fn test_mts() -> anyhow::Result<()> {
        let json_str = r#"
{
    "creatingLibrary": {
        "name": "MediaInfoLib",
        "version": "24.06",
        "url": "https://mediaarea.net/MediaInfo"
    },
    "media": {
        "@ref": "D:\\tmp\\00000.MTS",
        "track": [
            {
                "@type": "General",
                "ID": "0",
                "VideoCount": "1",
                "AudioCount": "1",
                "TextCount": "1",
                "FileExtension": "MTS",
                "Recorded_Date": "2013-11-09 15:07:11 UTC",
                "...": "..."
            },
            {
                "@type": "Video",
                "StreamOrder": "0-0",
                "ID": "4113",
                "MenuID": "1",
                "Format": "AVC",
                "Width": "1280",
                "Height": "720",
                "...": "..."
            },
            {
                "@type": "Audio",
                "StreamOrder": "0-1",
                "ID": "4352",
                "MenuID": "1",
                "Format": "AC-3",
                "...": "..."
            },
            {
                "@type": "Text",
                "StreamOrder": "0-2",
                "ID": "4608",
                "MenuID": "1",
                "...": "..."
            }
        ]
    }
}
"#;

        let metadata = super::extract_metadata_from_json(json_str)?;
        let expected = MetaData {
            width: 1280,
            height: 720,
            creation_date: Some(crate::parse_date("2013-11-09T15:07:11")),
        };
        assert_eq!(metadata, expected);
        Ok(())
    }

    #[test]
    fn test_jpg() -> anyhow::Result<()> {
        let json_str = r#"
{
  "creatingLibrary": {
    "name": "MediaInfoLib",
    "version": "24.06",
    "url": "https://mediaarea.net/MediaInfo"
  },
  "media": {
    "@ref": "test-data/sample-exif_1200x800_with_date.jpg",
    "track": [
      {
        "@type": "General",
        "ImageCount": "1",
        "FileExtension": "jpg",
        "Format": "JPEG",
        "FileSize": "116025",
        "StreamSize": "0",
        "File_Created_Date": "2024-10-27 13:47:23.630 UTC",
        "File_Created_Date_Local": "2024-10-27 14:47:23.630",
        "File_Modified_Date": "2024-10-27 13:47:23.630 UTC",
        "File_Modified_Date_Local": "2024-10-27 14:47:23.630"
      },
      {
        "@type": "Image",
        "Format": "JPEG",
        "Width": "1200",
        "Height": "800",
        "ColorSpace": "YUV",
        "ChromaSubsampling": "4:2:0",
        "BitDepth": "8",
        "Compression_Mode": "Lossy",
        "StreamSize": "116025"
      }
    ]
  }
}
"#;

        let metadata = super::extract_metadata_from_json(json_str)?;
        let expected = MetaData {
            width: 1200,
            height: 800,
            creation_date: None,
        };
        assert_eq!(metadata, expected);
        Ok(())
    }
}
