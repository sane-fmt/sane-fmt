use super::{read_dir::read_into, List};
use std::{io, path::PathBuf};

/// Default list of files to check.
pub fn default_files() -> io::Result<List> {
    let mut list = List::new();
    read_into(&mut list, &PathBuf::from("."))?;
    Ok(list)
}
