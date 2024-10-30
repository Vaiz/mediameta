use std::fs::File;
use std::io::BufReader;

#[test]
fn test_360() -> anyhow::Result<()> {
    let path = "test-data/sample-mkv-files-sample_640x360.mkv";
    let meta = mediameta::extract_file_metadata(path)?;

    let expected = mediameta::MetaData {
        width: 640,
        height: 360,
        creation_date: None,
    };
    assert_eq!(expected, meta);
    Ok(())
}

#[test]
fn test_540() -> anyhow::Result<()> {
    let path = "test-data/sample-mkv-files-sample_960x540.mkv";
    let meta = mediameta::extract_file_metadata(path)?;

    let expected = mediameta::MetaData {
        width: 960,
        height: 540,
        creation_date: None,
    };
    assert_eq!(expected, meta);
    Ok(())
}

#[test]
fn test_date() -> anyhow::Result<()> {
    let path = "test-data/sample-mkv-files-sample_640x360_with_date.mkv";
    let meta = mediameta::extract_file_metadata(path)?;

    let expected = mediameta::MetaData {
        width: 640,
        height: 360,
        creation_date: Some(super::parse_date("2011-04-17T17:33:45")),
    };
    assert_eq!(expected, meta);
    Ok(())
}

#[test]
fn test_wrong_extension() -> anyhow::Result<()> {
    let path = "test-data/sample-mkv-files-sample_640x360_with_date.test";
    assert!(mediameta::extract_file_metadata(path).is_err());

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let meta = mediameta::extract_mkv_metadata(reader)?;
    let expected = mediameta::MetaData {
        width: 640,
        height: 360,
        creation_date: Some(super::parse_date("2011-04-17T17:33:45")),
    };
    assert_eq!(expected, meta);
    Ok(())
}
