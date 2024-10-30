$ErrorActionPreference = "Stop"

cargo +nightly test
cargo +nightly test --features=image
cargo +nightly test --features=mediainfo
cargo +nightly test --all-features