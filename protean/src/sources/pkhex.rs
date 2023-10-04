use std::path::{Path, PathBuf};

mod personal_info;
pub use personal_info::*;

pub fn pkhex_resources_dir(source_dir: &Path) -> PathBuf {
    source_dir.join("PKHeX/PKHeX.Core/Resources/")
}

pub fn personal_info_path(source_dir: &Path, data_set: &str) -> PathBuf {
    pkhex_resources_dir(source_dir).join(format!("byte/personal/personal_{data_set}"))
}
