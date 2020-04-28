use super::super::When;
use std::path::Path;

/// Lookup a function that may print scanned filesystem objects according to color support.
/// * If `--color=never`, the returning function would do nothing.
/// * Otherwise, the returning function would print scanned filesystem objects.
///
/// The message (if any) would be deleted by `clear_current_line` afterward
pub fn get(color: When) -> fn(&Path) {
    if color == When::Never {
        |_| ()
    } else {
        |path| print!("scan {:?}", path)
    }
}
