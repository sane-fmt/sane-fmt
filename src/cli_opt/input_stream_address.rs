use pipe_trait::*;
use std::{path::PathBuf, str::FromStr};

#[derive(Debug, PartialEq, Clone)]
pub enum InputStreamAddress {
    Stdin,
    File(PathBuf),
}

impl FromStr for InputStreamAddress {
    type Err = String;

    fn from_str(path: &str) -> Result<Self, Self::Err> {
        Ok(if path == "-" {
            InputStreamAddress::Stdin
        } else {
            path.pipe(PathBuf::from).pipe(InputStreamAddress::File)
        })
    }
}
