use super::{read_dir::read_into, Error, Item, List};
use std::{fs::metadata, path::PathBuf};

/// Create a list of TypeScript/JavaScript files
///
/// * Convert directory path to a list of TypeScript/JavaScript files
/// * Keep file path as-is
pub fn create_list<Input>(files: Input) -> Result<List, Error>
where
    Input: Iterator<Item = PathBuf>,
{
    let mut result = List::new();

    for path in files {
        let file_type = metadata(&path)
            .map_err(|error| Error::new(path.clone(), error))?
            .file_type();

        if file_type.is_dir() {
            read_into(&mut result, &path)?;
        } else {
            result.push(Item { file_type, path });
        }
    }

    Ok(result)
}
