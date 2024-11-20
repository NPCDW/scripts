use std::fs;
use std::path::{Path, PathBuf};

pub fn read_file(path: &Path) -> anyhow::Result<String> {
    Ok(fs::read_to_string(path)?)
}

pub fn get_current_dir() -> PathBuf {
    std::env::current_dir().unwrap_or_else(|e| {
        panic!("Failed to get current path: {:?}", e);
    })
}
