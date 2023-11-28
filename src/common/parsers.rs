// use std::error::Error;
use color_eyre::eyre::{eyre, Result};
use std::path::PathBuf;

// TODO: setup CI on gitlab to run tests on every commit

// TODO: add tests
/// custom parser to validate that an arg is a directory
#[allow(dead_code)]
pub fn parse_dir(s: &str) -> Result<PathBuf> {
    let path = PathBuf::from(s);
    if path.is_dir() {
        Ok(path)
    } else {
        Err(eyre!(format!("{} is not a directory", s)))
    }
}

// TODO: add tests
/// custom parser to parse a crate-version input of the form
/// `<crate_name>:<crate_version>`
#[allow(dead_code)]
pub fn parse_name_version(s: &str) -> Result<(String, String)> {
    let parts = s
        .split_once(':')
        .ok_or_else(|| eyre!("invalid input (should be in the format <name>:<version>)"))?;
    Ok((String::from(parts.0), String::from(parts.1)))
}
