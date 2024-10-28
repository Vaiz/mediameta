use crate::MetaData;
use anyhow::{bail, Context};
use chrono::prelude::*;
use cmd_lib::run_fun;
use std::path::{Path, PathBuf};
use std::sync::{Arc, LazyLock};
use std::time::SystemTime;

static MEDIAINFO_PATH: LazyLock<Result<PathBuf, Arc<anyhow::Error>>> = LazyLock::new(|| {
    which::which("mediainfo")
        .with_context(|| "Cannot find mediainfo binary")
        .map_err(anyhow::Error::from)
        .map_err(Arc::from)
});
pub fn extract_metadata<P: AsRef<Path>>(file_path: P) -> anyhow::Result<MetaData> {
    let mediainfo = (*MEDIAINFO_PATH).as_ref();
    if mediainfo.is_err() {
        bail!(mediainfo.unwrap_err().to_string());
    }
    let mediainfo = mediainfo.unwrap();
    let file_path = file_path.as_ref();
    if !file_path.exists() {
        bail!("Cannot find file {}", file_path.to_string_lossy());
    }

    let result = run_fun!($mediainfo --Output=JSON $file_path)?;
    extract_metadata_from_json(&result)
}

fn extract_metadata_from_json(json: &str) -> anyhow::Result<MetaData> {
    let root: Root = serde_json::from_str(json)?;
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
                    metadata.creation_date = Some(parse_datetime(&recorded_date));
                } else if let Some(encoded_date) = general.encoded_date {
                    metadata.creation_date = Some(parse_datetime(&encoded_date));
                }
            }
            Track::Video(video) => {
                metadata.width = video.width.parse()?;
                metadata.height = video.height.parse()?;
                is_media = true;
            }
            Track::Image(image) => {
                metadata.width = image.width.parse()?;
                metadata.height = image.height.parse()?;
                is_media = true;
            }
            Track::Other => {}
        }
    }

    if !is_media {
        bail!("Failed to find any media metadata");
    }

    Ok(metadata)
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
    #[serde(rename = "Width")]
    width: String,
    #[serde(rename = "Height")]
    height: String,
}

#[derive(serde::Deserialize, Debug)]
struct ImageTrack {
    #[serde(rename = "Width")]
    width: String,
    #[serde(rename = "Height")]
    height: String,
}

#[derive(serde::Deserialize, Debug)]
struct Media {
    track: Vec<Track>,
}

#[derive(serde::Deserialize, Debug)]
struct Root {
    media: Media,
}

fn parse_datetime(datetime: &str) -> SystemTime {
    let datetime: DateTime<Utc> = NaiveDateTime::parse_from_str(&datetime, "%Y-%m-%d %H:%M:%S UTC")
        .with_context(|| format!("Failed to parse date {datetime}"))
        .unwrap()
        .and_utc();
    datetime.into()
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
