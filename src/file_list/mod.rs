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
