use super::super::{
    cross_platform_path,
    diff::diff_lines,
    term::color::*,
    DetailLevel::{self, *},
    LogFormat::{self, *},
};
use std::path::Path;

/// Log found filesystem object and maybe diff if `--details` is not `count`.
pub type Act<'a> = Box<dyn Fn(&Path, &str, &str) + 'a>;

/// Lookup a function that may log found filesystem object according to `--details`.
/// * If `--details=count`, the returning function would do nothing.
/// * If `--details=name`, the returning function would log names.
/// * If `--details=diff`, the returning function would log names and diffs.
pub fn get(details: DetailLevel, log_format: LogFormat, theme: &BoxedColorScheme) -> Act {
    let format_name = move |path: &Path| {
        let message = format!("✗ {}", cross_platform_path::to_string(path, '/'));
        format!("{}", theme.diff().paint(message))
    };
    let print_name = move |path: &Path| {
        println!("{}", format_name(path));
    };
    match (details, log_format) {
        (Count, _) => Box::new(|_, _, _| ()),
        (Name, Human) => Box::new(move |path, _, _| print_name(path)),
        (Name, GitHubActions) => Box::new(move |path, _, _| {
            println!("::error file={}::Format error", path.to_string_lossy());
            print_name(path);
        }),
        (Diff, Human) => Box::new(move |path, old, new| {
            print_name(path);
            for line in diff_lines(old, new, theme) {
                println!("{}", line);
            }
        }),
        (Diff, GitHubActions) => Box::new(move |path, old, new| {
            println!("::error file={}::Format error", path.to_string_lossy());
            println!("::group::{}", format_name(path));
            for line in diff_lines(old, new, theme) {
                println!("{}", line);
            }
            println!("::endgroup::");
        }),
    }
}
