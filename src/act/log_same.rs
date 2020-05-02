use super::super::{
    cross_platform_path,
    term::color::*,
    DetailLevel::{self, *},
};
use std::path::Path;

/// Log found filesystem object and maybe diff if `--details` is not `count`.
pub type Act<'a> = Box<dyn Fn(&Path) + 'a>;

/// Lookup a function that may log found filesystem object according to `--details`.
/// * If `--details=count`, the returning function would do nothing.
/// * If `--details=name`, the returning function would log names.
/// * If `--details=diff`, the returning function would log names and diffs.
pub fn get(details: DetailLevel, hide_passed: bool, theme: &'_ BoxedColorScheme) -> Act<'_> {
    let print_name = move |path: &Path| {
        let message = format!("🗸 {}", cross_platform_path::to_string(path, '/'));
        println!("{}", theme.same().paint(message));
    };
    match (details, hide_passed) {
        (Count, _) | (_, true) => Box::new(|_| ()),
        (Name, false) | (Diff, false) => Box::new(move |path| print_name(path)),
    }
}
