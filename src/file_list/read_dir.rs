use super::{Item, List};
use std::{
    fs::{metadata, read_dir},
    io,
    path::PathBuf,
};

pub const IGNORED_NAMES: &[&str] = &[".git", "node_modules"];
pub const EXTENSIONS: &[&str] = &["ts", "tsx", "js", "jsx"];

/// Add all applicable files in a directory into an existing list
pub fn read_into(list: &mut List, dirname: &PathBuf) -> io::Result<()> {
    let mut entries = read_dir(dirname)?
        .map(|entry| -> io::Result<_> {
            let entry = entry?;
            let path = entry.path();
            let file_type = metadata(&path)?.file_type();
            Ok((entry, path, file_type))
        })
        .collect::<io::Result<Vec<_>>>()?;
    entries.sort_by(|(_, a, _), (_, b, _)| a.cmp(b));
    for (entry, path, file_type) in entries {
        if file_type.is_dir() {
            let name = entry.file_name().to_string_lossy().to_string();
            if !IGNORED_NAMES.contains(&name.as_str()) {
                read_into(list, &dirname.join(name))?;
            }
        } else if file_type.is_file() {
            if let Some(extension) = path.extension() {
                if EXTENSIONS.contains(&extension.to_string_lossy().to_string().as_str())
                    && !path.to_string_lossy().ends_with(".d.ts")
                {
                    list.push(Item { path, file_type });
                }
            }
        }
    }

    Ok(())
}