use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn absoulte_path(file: &str) -> Option<String> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let cleaned = file.trim_start_matches('/');
    path.push("src");
    path.push("inputs");
    path.push(cleaned);
    Some(path.to_string_lossy().into_owned())
}
