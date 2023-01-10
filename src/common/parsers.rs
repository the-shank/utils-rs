use std::error::Error;
use std::path::PathBuf;

/// custom parser to validate that an arg is a directory
#[allow(dead_code)]
pub fn parse_dir(s: &str) -> Result<PathBuf, Box<dyn Error + Send + Sync + 'static>> {
    let path = PathBuf::from(s);
    if path.is_dir() {
        Ok(path)
    } else {
        Err(format!("{} is not a directory", s).into())
    }
}
