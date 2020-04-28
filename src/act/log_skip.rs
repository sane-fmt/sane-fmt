use super::super::DetailLevel;
use std::path::Path;

/// Lookup a function that may print skipped filesystem objects according to `--details`.
/// * If `--details=count`, the returning function would do nothing.
/// * Otherwise, the returning function would print skipped filesystem objects.
pub fn get(details: DetailLevel) -> fn(&Path) {
    if details == DetailLevel::Count {
        |_| ()
    } else {
        |path| println!("skip {:?} (not a file)", path)
    }
}
