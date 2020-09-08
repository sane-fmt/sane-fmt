use super::{Error, Item, List};
use std::{
    fs::{metadata, read_dir},
    path::PathBuf,
};

pub const IGNORED_NAMES: &[&str] = &[".git", "node_modules"];
pub const EXTENSIONS: &[&str] = &["ts", "tsx", "js", "jsx"];

/// Add all applicable files in a directory into an existing list
pub fn read_into(list: &mut List, dirname: &PathBuf) -> Result<(), Error> {
    let mut entries = read_dir(dirname)
        .map_err(|error| Error::new(dirname.clone(), error))?
        .map(|entry| -> Result<_, _> {
            let entry = entry.map_err(|error| Error::new(dirname.clone(), error))?;
            let path = entry.path();
            let file_type = metadata(&path)
                .map_err(|error| Error::new(path.clone(), error))?
                .file_type();
            Ok((entry, path, file_type))
        })
        .collect::<Result<Vec<_>, _>>()?;
    entries.sort_by(|(_, a, _), (_, b, _)| a.cmp(b));
    for (entry, path, file_type) in entries {
        if file_type.is_dir() {
            let name = entry.file_name().to_string_lossy().into_owned();
            if !IGNORED_NAMES.contains(&name.as_str()) {
                read_into(list, &dirname.join(name))?;
            }
        } else if file_type.is_file() {
            if let Some(extension) = path.extension() {
                if EXTENSIONS.contains(&extension.to_string_lossy().into_owned().as_str())
                    && !path.to_string_lossy().ends_with(".d.ts")
                {
                    list.push(Item { path, file_type });
                }
            }
        }
    }

    Ok(())
}
