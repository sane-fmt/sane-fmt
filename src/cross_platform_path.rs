use std::path::{Path, PathBuf};

/// Replace `/` and `\` in an iterator of characters.
pub fn convert_chars<Chars>(chars: Chars, separator: char) -> impl Iterator<Item = char>
where
    Chars: Iterator<Item = char>,
{
    chars.map(move |ch| match ch {
        '/' | '\\' => separator,
        _ => ch,
    })
}

/// Replace `/` and `\` in a string.
pub fn convert_string<Text: AsRef<str>>(text: Text, separator: char) -> String {
    convert_chars(text.as_ref().chars(), separator).collect()
}

/// Replace `/` and `\` in a path.
pub fn convert_path<P: AsRef<Path>>(path: P, separator: char) -> PathBuf {
    from_string(path.as_ref().to_string_lossy(), separator)
}

/// Replace `/` and `\` in a string and convert it into path.
pub fn from_string<Text: AsRef<str>>(text: Text, separator: char) -> PathBuf {
    PathBuf::from(convert_string(text, separator))
}

/// Replace `/` and `\` in a path and convert it to string
pub fn to_string<P: AsRef<Path>>(path: P, separator: char) -> String {
    convert_string(path.as_ref().to_string_lossy(), separator)
}
