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
            let indented_diff = add_indent(diff, "  ");
            println!("{}", indented_diff);
        },
    }
}

fn add_indent(text: String, indent: &str) -> String {
    text.split("\n")
        .map(|line| format!("{}{}", indent, line))
        .collect::<Vec<_>>()
        .join("\n")
}