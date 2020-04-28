use super::super::diff::diff_lines;
use super::super::DetailLevel::{self, *};
use std::path::Path;

/// Log found filesystem object and maybe diff if `--details` is not `count`.
pub type Act = fn(path: &Path, old: &String, new: &String);

/// Lookup a function that may log found filesystem object according to `--details`.
/// * If `--details=count`, the returning function would do nothing.
/// * If `--details=name`, the returning function would log names.
/// * If `--details=diff`, the returning function would log names and diffs.
pub fn get(details: DetailLevel) -> Act {
    match details {
        Count => |_, _, _| (),
        Name => |path, _, _| {
            println!("find {:?}", path);
        },
        Diff => |path, old, new| {
            println!("find {:?}", path);
            for line in diff_lines(old, new) {
                println!("{}", line);
            }
        },
    }
}
