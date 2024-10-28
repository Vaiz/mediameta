use video_info::mediainfo::extract_metadata;

#[test]
fn test_jpg_no_exif() -> anyhow::Result<()> {
    let path = "test-data/sample-no-exif_1200x800.jpg";
    let meta = extract_metadata(path)?;
    let expected = video_info::MetaData {
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
    let expected = video_info::MetaData {
        width: 1200,
        height: 800,
        creation_date: None,
    };
    assert_eq!(expected, meta);
    Ok(())
}