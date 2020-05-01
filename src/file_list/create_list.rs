use super::{Item, List};
use std::{fs::symlink_metadata, io, path::PathBuf};

pub fn create_list<Input>(files: Input) -> io::Result<List>
where
    Input: Iterator<Item = PathBuf>,
{
    let mut result = List::new();

    for path in files {
        result.push(Item {
            file_type: symlink_metadata(&path)?.file_type(),
            path,
        })
    }

    Ok(result)
}
