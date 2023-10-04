use std::path::{Path, PathBuf};

pub fn home_resources_dir(source_dir: &Path) -> PathBuf {
    source_dir.join("generated")
}

pub fn text_file_path(source_dir: &Path, data_set: &str, language: &str) -> PathBuf {
    home_resources_dir(source_dir).join(format!("{language}/{data_set}.txt"))
}
