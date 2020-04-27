use super::super::diff::diff_lines;
use super::super::DetailLevel::{self, *};
use std::path::Path;

pub type Act = fn(path: &Path, old: &String, new: &String);

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
