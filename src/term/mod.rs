pub mod color;

pub const CLEAR_CURRENT_LINE: &str = "\u{1b}[2K\r";

/// Emit ANSI sequence that clears a line which the cursor is in.
pub fn clear_current_line() {
    print!("{}", CLEAR_CURRENT_LINE);
}
