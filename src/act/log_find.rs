use super::super::diff::diff_lines;
use super::super::term::color::*;
use super::super::DetailLevel::{self, *};
use std::path::Path;

/// Log found filesystem object and maybe diff if `--details` is not `count`.
pub type Act<'a> = Box<dyn Fn(&Path, &str, &str) + 'a>;

/// Lookup a function that may log found filesystem object according to `--details`.
/// * If `--details=count`, the returning function would do nothing.
/// * If `--details=name`, the returning function would log names.
/// * If `--details=diff`, the returning function would log names and diffs.
pub fn get<'a>(details: DetailLevel, theme: &'a BoxedColorScheme) -> Act {
    let print_name = move |path: &Path| {
        let indicator = theme.find_indicator().paint("find");
        let name = theme
            .find_name()
            .paint(path.to_string_lossy().to_string().as_str());
        println!("{} {}", indicator, name);
    };
    match details {
        Count => Box::new(|_, _, _| ()),
        Name => Box::new(move |path, _, _| print_name(path)),
        Diff => Box::new(move |path, old, new| {
            print_name(path);
            for line in diff_lines(old, new) {
                println!("{}", line);
            }
        }),
    }
}
