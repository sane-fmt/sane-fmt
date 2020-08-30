//! GitHub Issue: https://github.com/sane-fmt/sane-fmt/issues/66

#![cfg(test)]
pub mod utils;
pub use utils::*;

use std::{fs::write, process::Command};

#[test]
fn test() {
    let file_name = tempfile::Builder::new()
        .prefix(TEMP_PREFIX)
        .suffix(TEMP_SUFFIX)
        .tempdir()
        .unwrap()
        .into_path()
        .join("file.js");

    write(&file_name, "const a = 'hello world';\n").unwrap();

    let output = Command::new(EXE)
        .arg("--write")
        .arg(file_name.to_string_lossy().to_string())
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "STDERR: {}",
        u8v_to_utf8(&output.stderr),
    );
}
