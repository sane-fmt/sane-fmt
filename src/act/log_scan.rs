use std::path::Path;
use super::super::When;

pub fn get(color: When) -> fn(&Path) {
    if color == When::Never {
        |_| ()
    } else {
        |path| print!("scan {:?}", path)
    }
}
