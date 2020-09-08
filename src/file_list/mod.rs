mod create_list;
mod default_list;
mod read_dir;
mod read_list;

pub use create_list::*;
pub use default_list::*;
pub use read_list::*;

use std::{fs::FileType, path::PathBuf};

pub struct Item {
    pub path: PathBuf,
    pub file_type: FileType,
}

pub type List = Vec<Item>;

#[derive(Debug, Clone)]
pub struct Error {
    pub path: PathBuf,
    pub message: String,
}
impl Error {
    fn new(path: PathBuf, message: impl ToString) -> Self {
        Error {
            path,
            message: message.to_string(),
        }
    }
}
impl ToString for Error {
    fn to_string(&self) -> String {
        format!("{}: {}", self.path.to_string_lossy(), self.message)
    }
}
