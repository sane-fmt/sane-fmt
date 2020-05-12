use super::term::color::BoxedColorScheme;
pub use difference::Changeset as Diff;
use difference::Difference::{self, *};
use std::fmt::Display;

/// Calculate changeset of two strings.
pub fn diff(old: &str, new: &str) -> Diff {
    Diff::new(old, new, "\n")
}

/// Emit printable lines of diff.
pub fn diff_lines<'a>(
    old: &str,
    new: &str,
    theme: &'a BoxedColorScheme,
    prefixes: (
        impl Display + Copy + 'a,
        impl Display + Copy + 'a,
        impl Display + Copy + 'a,
    ),
) -> impl Iterator<Item = String> + 'a {
    let (same, add, rem) = prefixes;
    let make_line = move |diff: Difference| match diff {
        Same(line) => theme.diff_line_same().paint(add_prefix(line, same)),
        Add(line) => theme.diff_line_add().paint(add_prefix(line, add)),
        Rem(line) => theme.diff_line_rem().paint(add_prefix(line, rem)),
    };
    diff(&old, &new)
        .diffs
        .into_iter()
        .map(make_line)
        .map(|line| format!("{}", line))
}

/// Add prefix to every line in a string.
fn add_prefix(text: String, prefix: impl Display) -> String {
    text.lines()
        .map(|line| format!("{}{}", prefix, line))
        .collect::<Vec<_>>()
        .join("\n")
}
