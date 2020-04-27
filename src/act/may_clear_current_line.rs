use super::super::term::clear_current_line;
use super::super::Ternary;

pub fn get(color: Ternary) -> fn() {
    if color == Ternary::Never {
        || ()
    } else {
        clear_current_line
    }
}
