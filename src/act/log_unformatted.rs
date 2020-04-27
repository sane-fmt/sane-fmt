use super::super::diff::diff_text;
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
            let diff = diff_text(old, new);
            println!("{}", diff);
        },
    }
}
