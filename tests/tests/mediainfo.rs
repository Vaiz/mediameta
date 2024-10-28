use mediameta::mediainfo::extract_metadata;

#[test]
fn test_jpg_no_exif() -> anyhow::Result<()> {
    let path = "test-data/sample-no-exif_1200x800.jpg";
    let meta = extract_metadata(path)?;
    let expected = mediameta::MetaData {
        width: 1200,
        height: 800,
        creation_date: None,
    };
    assert_eq!(expected, meta);
    Ok(())
}

#[test]
fn test_jpg_with_date() -> anyhow::Result<()> {
    let path = "test-data/sample-exif_1200x800_with_date.jpg";
    let meta = extract_metadata(path)?;
    let expected = mediameta::MetaData {
        width: 1200,
        height: 800,
        creation_date: None,
    };
    assert_eq!(expected, meta);
    Ok(())
}

#[test]
fn test_mkv_360() -> anyhow::Result<()> {
    let path = "test-data/sample-mkv-files-sample_640x360.mkv";
    let meta = extract_metadata(path)?;

    let expected = mediameta::MetaData {
        width: 640,
        height: 360,
        creation_date: None,
    };
    assert_eq!(expected, meta);
    Ok(())
}

#[test]
fn test_mkv_540() -> anyhow::Result<()> {
    let path = "test-data/sample-mkv-files-sample_960x540.mkv";
    let meta = extract_metadata(path)?;

    let expected = mediameta::MetaData {
        width: 960,
        height: 540,
        creation_date: None,
    };
    assert_eq!(expected, meta);
    Ok(())
}

#[test]
fn test_mkv_date() -> anyhow::Result<()> {
    let path = "test-data/sample-mkv-files-sample_640x360_with_date.mkv";
    let meta = extract_metadata(path)?;
    println!("{meta}");

    let expected = mediameta::MetaData {
        width: 640,
        height: 360,
        creation_date: Some(super::parse_date("2011-04-17T17:33:45")),
    };
    assert_eq!(expected, meta);
    Ok(())
}

#[test]
fn test_mp4_360() -> anyhow::Result<()> {
    let path = "test-data/sample-mp4-files-sample_640x360.mp4";
    let meta = extract_metadata(path)?;

    let expected = mediameta::MetaData {
        width: 640,
        height: 360,
        creation_date: None,
    };
    assert_eq!(expected, meta);
    Ok(())
}

#[test]
fn test_mp4_540() -> anyhow::Result<()> {
    let path = "test-data/sample-mp4-files-sample_960x540.mp4";
    let meta = extract_metadata(path)?;

    let expected = mediameta::MetaData {
        width: 960,
        height: 540,
        creation_date: None,
    };
    assert_eq!(expected, meta);
    Ok(())
}

#[test]
fn test_mp4_date() -> anyhow::Result<()> {
    let path = "test-data/sample-mp4-files-sample_640x360_with_date.mp4";
    let meta = extract_metadata(path)?;

    let expected = mediameta::MetaData {
        width: 640,
        height: 360,
        creation_date: Some(super::parse_date("2021-08-13T18:04:35")),
    };
    assert_eq!(expected, meta);
    Ok(())
}

#[test]
fn test_txt_file() {
    let path = "test-data/source.txt";
    let meta = extract_metadata(path);
    assert!(meta.is_err());
}
