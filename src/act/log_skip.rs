use super::super::DetailLevel;
use std::path::Path;

pub fn get(details: DetailLevel) -> fn(&Path) {
    if details == DetailLevel::Count {
        |_| ()
    } else {
        |path| println!("skip {:?} (not a file)", path)
    }
}
