use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, Read, Seek};
use std::path::Path;
use std::time::{Duration, SystemTime};
//use matroska::MatroskaReader;
use anyhow::Context;
use mp4::Mp4Track;

#[derive(Debug)]
struct MetaData {
    width: u16,
    height: u16,
    size: u64,
    creation_date: SystemTime,
}

pub fn extract_metadata(file_path: &str) -> anyhow::Result<MetaData> {
    let path = Path::new(file_path);
    let file_extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();

    match file_extension.as_str() {
        "mp4" => extract_mp4_metadata(file_path),
        "mkv" => extract_mkv_metadata(file_path),
        _ => anyhow::bail!("Unsupported file format: {}", file_extension)
    }
}

fn extract_mp4_metadata(file_path: &str) -> anyhow::Result<MetaData> {
    let file = File::open(file_path).with_context(|| format!("Failed to open file: {}", file_path))?;
    let size = file.metadata()?.len();
    let reader = BufReader::new(file);
    let mp4 = mp4::Mp4Reader::read_header(reader, size)
        //let mp4 = mp4_read_header(reader, size)
        .with_context(|| "Failed to read MP4 header")?;

    let video_track = find_video_track(mp4.tracks());
    let (width, height) = if let Some(track) = video_track {
        (track.width(), track.height())
    } else {
        (0,0)
    };

    // Retrieve creation time from the MP4 metadata
    let creation_date = {
        // MP4 creation time is based on seconds since 1904-01-01
        let mp4_epoch: SystemTime = std::time::UNIX_EPOCH - Duration::from_secs(2_082_844_800);
        mp4_epoch + Duration::from_secs(mp4.moov.mvhd.creation_time)
    };

    Ok(MetaData {
        width,
        height,
        size,
        creation_date,
    })
}

fn find_video_track(tracks: &HashMap<u32, Mp4Track>) -> Option<&mp4::Mp4Track> {
    for (_, track) in tracks {
        match track.track_type() {
            Ok(mp4::TrackType::Video) => return Some(track),
            _ => {}
        }
    }
    None
}
/*
#[derive(Debug)]
pub struct Mp4Reader2<R> {
    reader: R,
    pub ftyp: mp4::FtypBox,
    pub moov: mp4::MoovBox,
    pub moofs: Vec<mp4::MoofBox>,
    pub emsgs: Vec<mp4::EmsgBox>,

    tracks: HashMap<u32, mp4::Mp4Track>,
    size: u64,
}

pub fn mp4_read_header<R: Read + Seek>(mut reader: R, size: u64) -> anyhow::Result<Mp4Reader2<R>> {
    use mp4::*;

    let start = reader.stream_position()?;

    let mut ftyp = None;
    let mut moov = None;
    let mut moofs = Vec::new();
    let mut emsgs = Vec::new();

    let mut current = start;
    while current < size {
        // Get box header.
        let header = BoxHeader::read(&mut reader)?;
        let BoxHeader { name, size: s } = header;
        if s > size {
            return Err(Error::InvalidData(
                "file contains a box with a larger size than it",
            ).into());
        }

        // Break if size zero BoxHeader, which can result in dead-loop.
        if s == 0 {
            break;
        }

        // Match and parse the atom boxes.
        match name {
            BoxType::FtypBox => {
                ftyp = Some(FtypBox::read_box(&mut reader, s).with_context(|| "Failed to read FtypBox")?);
            }
            BoxType::FreeBox => {
                skip_box(&mut reader, s).with_context(|| "Failed to skip FreeBox")?;
            }
            BoxType::MdatBox => {
                skip_box(&mut reader, s).with_context(|| "Failed to skip MdatBox")?;
            }
            BoxType::MoovBox => {
                moov = Some(MoovBox::read_box(&mut reader, s).with_context(|| "Failed to read MoovBox")?);
            }
            BoxType::MoofBox => {
                let moof = MoofBox::read_box(&mut reader, s).with_context(|| "Failed to read MoofBox")?;
                moofs.push(moof);
            }
            BoxType::EmsgBox => {
                let emsg = EmsgBox::read_box(&mut reader, s).with_context(|| "Failed to read EmsgBox")?;
                emsgs.push(emsg);
            }
            _ => {
                // XXX warn!()
                skip_box(&mut reader, s).with_context(|| format!("Failed to skip {name}"))?;
            }
        }
        current = reader.stream_position()?;
    }

    if ftyp.is_none() {
        return Err(Error::BoxNotFound(BoxType::FtypBox).into());
    }
    if moov.is_none() {
        return Err(Error::BoxNotFound(BoxType::MoovBox).into());
    }

    let size = current - start;
    let mut tracks = if let Some(ref moov) = moov {
        if moov.traks.iter().any(|trak| trak.tkhd.track_id == 0) {
            return Err(Error::InvalidData("illegal track id 0").into());
        }
        moov.traks
            .iter()
            .map(|trak| (trak.tkhd.track_id,
                         Mp4Track {
                             trak: trak.clone(),
                             trafs: Vec::new(),
                             default_sample_duration: 0,
                         }))
            .collect()
    } else {
        std::collections::HashMap::new()
    };

    // Update tracks if any fragmented (moof) boxes are found.
    if !moofs.is_empty() {
        let mut default_sample_duration = 0;
        if let Some(ref moov) = moov {
            if let Some(ref mvex) = &moov.mvex {
                default_sample_duration = mvex.trex.default_sample_duration
            }
        }

        for moof in moofs.iter() {
            for traf in moof.trafs.iter() {
                let track_id = traf.tfhd.track_id;
                if let Some(track) = tracks.get_mut(&track_id) {
                    track.default_sample_duration = default_sample_duration;
                    track.trafs.push(traf.clone())
                } else {
                    return Err(Error::TrakNotFound(track_id).into());
                }
            }
        }
    }

    Ok(Mp4Reader2 {
        reader,
        ftyp: ftyp.unwrap(),
        moov: moov.unwrap(),
        moofs,
        emsgs,
        size,
        tracks,
    })
}
*/
fn extract_mkv_metadata(file_path: &str) -> anyhow::Result<MetaData> {
    unimplemented!()
    /*
    let file = File::open(file_path).with_context(|| format!("Failed to open file: {}", file_path))?;
    let reader = BufReader::new(file);
    let matroska = MatroskaReader::new(reader).with_context(|| "Failed to read MKV file")?;

    // Extract width and height from the video track
    let track = matroska.tracks().iter().find(|t| t.is_video());
    let (width, height) = if let Some(video_track) = track {
        let width = video_track.video().map(|v| v.pixel_width).unwrap_or(0);
        let height = video_track.video().map(|v| v.pixel_height).unwrap_or(0);
        (width, height)
    } else {
        (0, 0) // default values if no video track is found
    };

    // Retrieve creation date from MKV metadata (Matroska files may have a date field in the Segment Info)
    let creation_date = matroska.segment().info().and_then(|info| {
        info.date_utc().map(|date| {
            // Convert from nanoseconds to `SystemTime`
            UNIX_EPOCH + Duration::from_nanos(date as u64)
        })
    });

    Ok(MetaData {
        width,
        height,
        size,
        creation_date,
    })

     */
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> anyhow::Result<()> {
        let file_path = "D:/tmp/1.mp4"; // Replace with your file path
        println!("{:?}", extract_metadata(file_path)?);
        Ok(())
    }
}
