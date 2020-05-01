use std::path::PathBuf;

/// Replace `/` and `\` in a path with valid separator.
pub fn from_string(text: &str, separator: char) -> PathBuf {
    PathBuf::from(
        text.chars()
            .map(|ch| match ch {
                '/' | '\\' => separator,
                _ => ch,
            })
            .collect::<String>(),
    )
}
