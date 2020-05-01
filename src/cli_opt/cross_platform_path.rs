use std::path::{PathBuf, MAIN_SEPARATOR};

/// Replace `/` and `\` in a path with valid separator.
pub fn from_string(text: &str) -> PathBuf {
    PathBuf::from(
        text.chars()
            .map(|ch| match ch {
                '/' | '\\' => MAIN_SEPARATOR,
                _ => ch,
            })
            .collect::<String>(),
    )
}
