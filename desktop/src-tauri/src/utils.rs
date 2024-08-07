use eyre::{Context, ContextCompat, Result};
use rand::distributions::Alphanumeric;
use rand::Rng;
use std::env;
use std::path::PathBuf;

pub fn random_string(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

pub fn get_current_dir() -> Result<PathBuf> {
    let current_dir = env::current_exe().context("current_dir")?;
    let current_dir = current_dir.parent().context("current dir parent")?;
    Ok(current_dir.to_path_buf())
}
