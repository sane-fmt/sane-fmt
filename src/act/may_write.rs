use std::{fs, io, path::Path};

/// Overwrite unformatted file if `--write` is present.
pub type Act = fn(&Path, &str) -> io::Result<()>;

/// Lookup a function that may write unformatted file.
/// * If `--write` is present, the returning function would overwrite
///   unformatted files with formatted content.
/// * Otherwise, the returning function would do nothing
pub fn get(write: bool) -> Act {
    if write {
        |path, content| fs::write(path, content)
    } else {
        |_, _| Ok(())
    }
}
