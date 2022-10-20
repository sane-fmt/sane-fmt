use super::term::color::ColorScheme;
use derive_more::Display;
use pipe_trait::*;
use similar::ChangeTag;
use similar::TextDiff;
use std::fmt::Display;
use yansi::Paint;

/// Differences between two strings.
pub struct Diff<'a>(TextDiff<'a, 'a, 'a, str>);

impl<'a> Diff<'a> {
    /// Compute the differences between two strings.
    pub fn new(old: &'a str, new: &'a str) -> Self {
        TextDiff::from_lines(old, new).pipe(Diff)
    }

    /// Emit printable lines of differences.
    pub fn lines<Prefix: Display + Copy + 'a>(
        &'a self,
        theme: &'a dyn ColorScheme,
        prefixes: (Prefix, Prefix, Prefix),
    ) -> impl Iterator<Item = DiffLine<'a, Prefix>> + 'a {
        let (equal, insert, delete) = prefixes;
        let painted_prefixed = move |(tag, value)| match tag {
            ChangeTag::Equal => theme.diff_line_equal().paint(Prefixed::new(equal, value)),
            ChangeTag::Insert => theme.diff_line_insert().paint(Prefixed::new(insert, value)),
            ChangeTag::Delete => theme.diff_line_delete().paint(Prefixed::new(delete, value)),
        };
        self.0
            .iter_all_changes()
            .map(|change| (change.tag(), change.value()))
            .map(|(tag, value)| (tag, value.strip_suffix('\n').unwrap_or(value)))
            .map(painted_prefixed)
            .map(DiffLine)
    }
}

/// Item of [`Diff::lines`].
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
