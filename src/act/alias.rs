use std::path::Path;
pub type Act<Return> = fn(path: &Path, old: String, new: String) -> Return;
