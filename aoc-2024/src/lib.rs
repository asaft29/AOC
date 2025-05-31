use anyhow::Result;
use std::{fs::read_to_string, path::PathBuf, str::FromStr};

pub fn absoulte_path(file: &str) -> Option<String> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let cleaned = file.trim_start_matches('/');
    path.push("src");
    path.push("inputs");
    path.push(cleaned);
    Some(path.to_string_lossy().into_owned())
}

// had a problem with char parsing so i made another function, will think of something better later
pub fn read_input_numbers<T: FromStr>(input: impl AsRef<str>) -> Result<Vec<Vec<T>>> {
    Ok(read_to_string(input.as_ref())?
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|word| word.parse::<T>().ok())
                .collect()
        })
        .collect())
}

pub fn read_input_chars(input: impl AsRef<str>) -> Result<Vec<Vec<char>>> {
    let content = read_to_string(input.as_ref())?;
    Ok(content.lines().map(|line| line.chars().collect()).collect())
}
