use super::{read_dir::read_into, Item, List};
use std::{fs::metadata, io, path::PathBuf};

/// Create a list of TypeScript/JavaScript files
///
/// * Convert directory path to a list of TypeScript/JavaScript files
/// * Keep file path as-is
pub fn create_list<Input>(files: Input) -> io::Result<List>
where
    Input: Iterator<Item = PathBuf>,
{
    let mut result = List::new();

    for path in files {
        let file_type = metadata(&path)?.file_type();

        if file_type.is_dir() {
            read_into(&mut result, &path)?;
        } else {
            result.push(Item { file_type, path });
        }
    }

    Ok(result)
}
