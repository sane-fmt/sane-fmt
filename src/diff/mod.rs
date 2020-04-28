pub use difference::Changeset as Diff;
use difference::Difference::*;

/// Calculate changeset of two strings.
pub fn diff(old: &String, new: &String) -> Diff {
    Diff::new(old.as_str(), new.as_str(), "\n")
}

/// Emit printable lines of diff.
pub fn diff_lines(old: &str, new: &str) -> impl Iterator<Item = String> {
    let old_r = &old.to_owned();
    let new_r = &new.to_owned();
    diff(old_r, new_r).diffs.into_iter().map(|diff| match diff {
        Same(line) => add_prefix(line, "   "),
        Add(line) => add_prefix(line, "  +"),
        Rem(line) => add_prefix(line, "  -"),
    })
}

/// Add prefix to every line in a string.
fn add_prefix(text: String, prefix: &str) -> String {
    text.split("\n")
        .map(|line| format!("{}{}", prefix, line))
        .collect::<Vec<_>>()
        .join("\n")
}
