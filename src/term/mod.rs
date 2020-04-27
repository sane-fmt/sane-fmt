pub static CLEAR_CURRENT_LINE: &str = "\u{1b}[2K\r";

pub fn clear_current_line() {
    print!("{}", CLEAR_CURRENT_LINE);
}
