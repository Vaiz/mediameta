#[test]
fn test_display() {
    let meta = mediameta::MetaData {
        width: 640,
        height: 360,
        creation_date: Some(super::parse_date("2021-08-13T18:04:35")),
    };

    let meta_str = format!("{meta}");
    assert_eq!(
        meta_str,
        "width: 640, height: 360, creation_date: 2021-08-13T18:04:35+00:00"
    );
}
