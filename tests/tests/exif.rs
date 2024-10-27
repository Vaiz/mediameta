#[test]
fn test_no_exif() {
    let path = "test-data/sample-no-exif_1200x800.jpg";
    assert!(video_info::extract_file_metadata(path).is_err());
}

#[test]
fn test_date() -> anyhow::Result<()> {
    let path = "test-data/sample-exif_1200x800_with_date.jpg";
    let meta = video_info::extract_file_metadata(path)?;

    #[cfg(not(feature = "image"))]
    let expected = video_info::MetaData {
        width: 0,
        height: 0,
        creation_date: Some(super::parse_date("2015-07-16T13:34:48")),
    };
    #[cfg(feature = "image")]
    let expected = video_info::MetaData {
        width: 1200,
        height: 800,
        creation_date: Some(super::parse_date("2015-07-16T13:34:48")),
    };
    assert_eq!(expected, meta);
    Ok(())
}
