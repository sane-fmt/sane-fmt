use super::super::term::clear_current_line;
use super::super::When;

/// Lookup a function that may clear the current line according color support.
/// * If `--color=never`, the returning function would do nothing.
/// * Otherwise, the returning function would print an ANSI sequence to clear a line.
pub fn get(color: When) -> fn() {
    if color == When::Never {
        || ()
    } else {
        clear_current_line
    }
}
