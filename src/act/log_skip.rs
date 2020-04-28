use super::super::{term::color::*, DetailLevel};
use std::path::Path;

/// Lookup a function that may print skipped filesystem objects according to `--details`.
/// * If `--details=count`, the returning function would do nothing.
/// * Otherwise, the returning function would print skipped filesystem objects.
pub fn get<'a>(details: DetailLevel, theme: &'a BoxedColorScheme) -> Box<dyn Fn(&Path) + 'a> {
    if details == DetailLevel::Count {
        Box::new(|_| ())
    } else {
        Box::new(move |path| {
            let indicator = theme.skip().paint("•");
            let path_str = path.to_string_lossy().to_string();
            let name = theme.skip_name().paint(path_str.as_str());
            let reason = theme.skip().paint("(not a file)");
            println!("{} {} {}", indicator, name, reason);
        })
    }
}
