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
            let styled_path = theme
                .skip_name()
                .paint(path.to_string_lossy().to_string().as_str());
            let message = format!("skip {} (not a file)", styled_path);
            let styled_message = theme.skip().paint(message.as_str());
            println!("{}", styled_message);
        })
    }
}
