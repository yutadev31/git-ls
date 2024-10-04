use std::{fs, path::PathBuf};

use anyhow::{Context, Result};

pub fn home_dir_mark(path: &str) -> Result<String> {
    let home_dir = dirs::home_dir().context("Failed to get home directory")?;
    let path = path.replace(home_dir.to_str().unwrap(), "~");

    Ok(path)
}

pub fn get_dir_items(path: &str) -> Result<Vec<PathBuf>> {
    let dir = fs::read_dir(path).context("Failed to read directory")?;
    let mut files: Vec<PathBuf> = Vec::new();

    for item in dir.into_iter() {
        let item = item.unwrap().path();
        files.push(item);
    }

    Ok(files)
}
