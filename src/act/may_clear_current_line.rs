use super::super::term::clear_current_line;
use super::super::When;

pub fn get(color: When) -> fn() {
    if color == When::Never {
        || ()
    } else {
        clear_current_line
    }
}
