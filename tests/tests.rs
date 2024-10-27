use std::time::SystemTime;

#[path = "tests/metadata.rs"]
mod metadata;
#[path = "tests/mkv.rs"]
mod mkv;
#[path = "tests/mp4.rs"]
mod mp4;

fn parse_date(date: &str) -> SystemTime {
    use chrono::prelude::*;
    use std::time::{Duration, UNIX_EPOCH};

    let naive_datetime =
        NaiveDateTime::parse_from_str(date, "%Y-%m-%dT%H:%M:%S").expect("Failed to parse date");

    UNIX_EPOCH + Duration::from_secs(naive_datetime.and_utc().timestamp() as u64)
}
