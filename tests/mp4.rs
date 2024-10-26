#[test]
fn test_360() -> anyhow::Result<()> {
    let path = "test-data/sample-mp4-files-sample_640x360.mp4";
    let meta = video_info::extract_file_metadata(path)?;

    let expected = video_info::MetaData {
        width: 640,
        height: 360,
        creation_date: None,
    };
    assert_eq!(expected, meta);
    Ok(())
}

#[test]
fn test_540() -> anyhow::Result<()> {
    let path = "test-data/sample-mp4-files-sample_960x540.mp4";
    let meta = video_info::extract_file_metadata(path)?;

    let expected = video_info::MetaData {
        width: 960,
        height: 540,
        creation_date: None,
    };
    assert_eq!(expected, meta);
    Ok(())
}