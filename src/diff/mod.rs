use super::term::color::BoxedColorScheme;
pub use difference::Changeset as Diff;
use difference::Difference::{self, *};

/// Calculate changeset of two strings.
pub fn diff(old: &str, new: &str) -> Diff {
    Diff::new(old, new, "\n")
}

/// Emit printable lines of diff.
pub fn diff_lines<'a>(
    old: &str,
    new: &str,
    theme: &'a BoxedColorScheme,
) -> impl Iterator<Item = String> + 'a {
    let make_line = move |diff: Difference| match diff {
        Same(line) => theme.diff_line_same().paint(add_prefix(line, "   ")),
        Add(line) => theme.diff_line_add().paint(add_prefix(line, "  +")),
        Rem(line) => theme.diff_line_rem().paint(add_prefix(line, "  -")),
    };
    diff(&old, &new)
        .diffs
        .into_iter()
        .map(make_line)
        .map(|line| format!("{}", line))
}

/// Add prefix to every line in a string.
fn add_prefix(text: String, prefix: &str) -> String {
    text.split('\n')
        .map(|line| format!("{}{}", prefix, line))
        .collect::<Vec<_>>()
        .join("\n")
}
