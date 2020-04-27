use super::super::When;
use std::path::Path;

pub fn get(color: When) -> fn(&Path) {
    if color == When::Never {
        |_| ()
    } else {
        |path| print!("scan {:?}", path)
    }
}
