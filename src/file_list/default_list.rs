use super::{read_dir::read_into, Error, List};
use std::path::PathBuf;

/// Default list of files to check.
pub fn default_files() -> Result<List, Error> {
    let mut list = List::new();
    read_into(&mut list, &PathBuf::from("."))?;
    Ok(list)
}
