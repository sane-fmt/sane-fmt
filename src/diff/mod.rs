pub use difference::Changeset as Diff;
use difference::Difference::*;

pub fn diff(old: &String, new: &String) -> Diff {
    Diff::new(old.as_str(), new.as_str(), "\n")
}

pub fn diff_lines(old: &String, new: &String) -> impl Iterator<Item = String> {
    diff(old, new).diffs.into_iter().map(|diff| match diff {
        Same(line) => add_prefix(line, "   "),
        Add(line) => add_prefix(line, "  +"),
        Rem(line) => add_prefix(line, "  -"),
    })
}

fn add_prefix(text: String, prefix: &str) -> String {
    text.split("\n")
        .map(|line| format!("{}{}", prefix, line))
        .collect::<Vec<_>>()
        .join("\n")
}
