use super::Act;
use super::super::DetailLevel::{self, *};

pub fn get(details: DetailLevel) -> Act<()> {
    match details {
        Count => |_, _, _| (),
        Name => |path, _, _| {
            println!("fmt {:?}", path);
        },
        Diff => |path, _old, _new| {
            println!("fmt {:?}", path);
            // TODO: show diff
        },
    }
}
