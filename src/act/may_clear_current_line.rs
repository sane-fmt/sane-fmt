use super::super::term::clear_current_line;

pub fn get(color: bool) -> fn() {
    if color {
        clear_current_line
    } else {
        || ()
    }
}
