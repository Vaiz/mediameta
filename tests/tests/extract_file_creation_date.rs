use mediameta::extract_file_creation_date;

#[test]
fn test_jpg_no_exif() {
    let path = "test-data/sample-no-exif_1200x800.jpg";
    assert!(extract_file_creation_date(path).is_err());
}

#[test]
fn test_jpg_with_date() -> anyhow::Result<()> {
    let path = "test-data/sample-exif_1200x800_with_date.jpg";
    let creation_date = extract_file_creation_date(path)?;
    let expected = super::parse_date("2015-07-16T13:34:48");
    assert_eq!(expected, creation_date);
    Ok(())
}

#[test]
fn test_mkv_360() {
    let path = "test-data/sample-mkv-files-sample_640x360.mkv";
    assert!(extract_file_creation_date(path).is_err());
}

#[test]
fn test_mkv_date() -> anyhow::Result<()> {
    let path = "test-data/sample-mkv-files-sample_640x360_with_date.mkv";
    let creation_date = extract_file_creation_date(path)?;
    let expected = super::parse_date("2011-04-17T17:33:45");
    assert_eq!(expected, creation_date);
    Ok(())
}

#[test]
fn test_mp4_360() {
    let path = "test-data/sample-mp4-files-sample_640x360.mp4";
    assert!(extract_file_creation_date(path).is_err());
}

#[test]
fn test_mp4_date() -> anyhow::Result<()> {
    let path = "test-data/sample-mp4-files-sample_640x360_with_date.mp4";
    let creation_date = extract_file_creation_date(path)?;
    let expected = super::parse_date("2021-08-13T18:04:35");
    assert_eq!(expected, creation_date);
    Ok(())
}

#[test]
fn test_txt_file() {
    let path = "test-data/source.txt";
    assert!(extract_file_creation_date(path).is_err());
}

#[cfg(not(feature = "mediainfo"))]
#[test]
fn test_wrong_extension() {
    let path = "test-data/sample-mkv-files-sample_640x360_with_date.test";
    assert!(extract_file_creation_date(path).is_err());
}

#[cfg(feature = "mediainfo")]
#[test]
fn test_wrong_extension() -> anyhow::Result<()> {
    let path = "test-data/sample-mkv-files-sample_640x360_with_date.test";
    let creation_date = extract_file_creation_date(path)?;
    let datetime: chrono::DateTime<chrono::Utc> = creation_date.into();
    println!("{}", datetime.to_rfc3339());
    let expected = super::parse_date("2011-04-17T17:33:45");
    assert_eq!(expected, creation_date);
    Ok(())
}
