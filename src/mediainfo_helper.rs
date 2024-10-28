use crate::MetaData;
use anyhow::Context;
use cmd_lib::run_fun;
use std::path::{Path, PathBuf};
use std::sync::{Arc, LazyLock};

static MEDIAINFO_PATH: LazyLock<Result<PathBuf, Arc<anyhow::Error>>> = LazyLock::new(|| {
    which::which("mediainfo")
        .with_context(|| "Cannot find mediainfo binary")
        .map_err(anyhow::Error::from)
        .map_err(Arc::from)
});
pub fn extract_metadata<P: AsRef<Path>>(file_path: P) -> anyhow::Result<MetaData> {
    let mediainfo = (*MEDIAINFO_PATH).as_ref();
    if mediainfo.is_err() {
        anyhow::bail!(mediainfo.unwrap_err().to_string());
    }
    let mediainfo = mediainfo.unwrap();
    let file_path = file_path.as_ref();
    if !file_path.exists() {
        anyhow::bail!("Cannot find file {}", file_path.to_string_lossy());
    }
    println!("path: {}", file_path.to_string_lossy());
    let result = run_fun!($mediainfo --Output=JSON $file_path)?;
    println!("result: {result}");
    let json = serde_json::from_str(&result)?;
    unimplemented!()
}
