pub use difference::Changeset as Diff;

pub fn diff(old: String, new: String) -> Diff {
    Diff::new(old.as_str(), new.as_str(), "\n")
}

pub fn diff_text(old: String, new: String) -> String {
    format!("{}", diff(old, new))
}
