pub use similar::TextDiff as Diff;

use super::term::color::ColorScheme;
use derive_more::Display;
use similar::ChangeTag;
use std::fmt::Display;
use yansi::Paint;

/// Calculate changeset of two strings.
pub fn diff<'a>(old: &'a str, new: &'a str) -> Diff<'a, 'a, 'a, str> {
    Diff::from_lines(old, new)
}

/// Emit printable lines of diff.
pub fn diff_lines<'a, Prefix: Display + Copy + 'a>(
    old: &'a str,
    new: &'a str,
    theme: &'a dyn ColorScheme,
    prefixes: (Prefix, Prefix, Prefix),
) -> Vec<DiffLine<'a, Prefix>> {
    let (equal, insert, delete) = prefixes;
    let painted_prefixed = |tag, value| match tag {
        ChangeTag::Equal => theme.diff_line_equal().paint(Prefixed::new(equal, value)),
        ChangeTag::Insert => theme.diff_line_insert().paint(Prefixed::new(insert, value)),
        ChangeTag::Delete => theme.diff_line_delete().paint(Prefixed::new(delete, value)),
    };
    diff(old, new)
        .iter_all_changes()
        .map(|change| (change.tag(), change.value()))
        .map(|(tag, value)| (tag, value.strip_suffix('\n').unwrap_or(value)))
        .map(|(tag, value)| painted_prefixed(tag, value))
        .map(DiffLine)
        .collect()
}

#[derive(Debug, Display)]
pub struct DiffLine<'a, Prefix: Display>(Paint<Prefixed<'a, Prefix>>);

#[derive(Debug, Display)]
#[display(fmt = "{prefix}{value}")]
struct Prefixed<'a, Prefix: Display> {
    prefix: Prefix,
    value: &'a str,
}

impl<'a, Prefix: Display> Prefixed<'a, Prefix> {
    fn new(prefix: Prefix, value: &'a str) -> Self {
        Self { prefix, value }
    }
}
