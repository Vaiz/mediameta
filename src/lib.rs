mod mp4_helper;
mod mkv_helper;

use anyhow::Context;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::path::Path;
use std::time::SystemTime;

#[derive(Debug, PartialEq)]
pub struct MetaData {
    pub width: u64,
    pub height: u64,
    pub creation_date: Option<SystemTime>,
}

#[derive(Debug, PartialEq)]
pub enum ContainerType {
    Mp4,
    Mkv,
}

fn get_container_type(file_path: &Path) -> anyhow::Result<ContainerType> {
    let file_extension = file_path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();

    match file_extension.as_str() {
        "mp4" => Ok(ContainerType::Mp4),
        "mkv" => Ok(ContainerType::Mkv),
        _ => anyhow::bail!("Unsupported container format: {}", file_extension)
    }
}

pub fn extract_file_metadata(file_path: &str) -> anyhow::Result<MetaData> {
    let path = Path::new(file_path);
    let container_type = get_container_type(path)?;
    let file = File::open(file_path).with_context(|| format!("Failed to open file: {}", file_path))?;
    let size = file.metadata()?.len();
    let reader = BufReader::new(file);
    extract_metadata(reader, size, container_type)
}

pub fn extract_metadata<R>(io: R, file_size: u64, container_type: ContainerType) -> anyhow::Result<MetaData>
where
    R: io::Read + io::Seek,
{
    match container_type {
        ContainerType::Mp4 => { mp4_helper::extract_mp4_metadata(io, file_size) }
        ContainerType::Mkv => { mkv_helper::extract_mkv_metadata(io) }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> anyhow::Result<()> {
        let file_path = "D:/tmp/1.mp4"; // Replace with your file path
        println!("{:?}", extract_file_metadata(file_path)?);
        Ok(())
    }
}
