use super::{Item, List};
use std::{
    fs::{read_dir, symlink_metadata},
    io,
    path::PathBuf,
};

pub const IGNORED_NAMES: &[&str] = &[".git", "node_modules"];
pub const EXTENSIONS: &[&str] = &["ts", "tsx", "js", "jsx"];

fn add_files(list: &mut List, dirname: &PathBuf) -> io::Result<()> {
    for res in read_dir(dirname)? {
        let entry = res?;
        let path = entry.path();
        let file_type = symlink_metadata(&path)?.file_type();
        if file_type.is_dir() {
            let name_b = entry.file_name().to_string_lossy().to_string();
            let name = name_b.as_str();
            if !IGNORED_NAMES.contains(&name) {
                add_files(list, &dirname.join(name))?;
            }
        } else if file_type.is_file() {
            if let Some(extension) = path.extension() {
                if EXTENSIONS.contains(&extension.to_string_lossy().to_string().as_str()) {
                    list.push(Item { path, file_type });
                }
            }
        }
    }

    Ok(())
}

/// Default list of files to check.
pub fn default_files() -> io::Result<List> {
    let mut list = List::new();
    add_files(&mut list, &PathBuf::from("."))?;
    Ok(list)
}
