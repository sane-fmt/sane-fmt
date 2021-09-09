#![cfg(test)]
pub mod utils;
pub use utils::*;

use pretty_assertions::assert_eq;

#[test]
fn stdin_files() {
    let output = Exe::fixtures().run_with_stdio(
        include_bytes!("./assets/include-files.input.txt"),
        &["-I", "-", "--color=never"],
    );
    assert_eq!(
        (
            u8v_to_utf8(&output.stdout),
            u8v_to_utf8(&output.stderr),
            output.status.success(),
        ),
        (
            include_str!("./expected-output/include-files.stdout.txt"),
            "ERROR: There are 1 unformatted files\n",
            false,
        ),
    );
}

#[test]
fn stdin_dirs() {
    let output = Exe::fixtures().run_with_stdio(
        include_bytes!("./assets/include-dirs.input.txt"),
        &["-I", "-", "--color=never"],
    );
    assert_eq!(
        (
            u8v_to_utf8(&output.stdout),
            u8v_to_utf8(&output.stderr),
            output.status.success()
        ),
        (
            include_str!("./expected-output/include-dirs.stdout.txt"),
            "ERROR: There are 5 unformatted files\n",
            false,
        ),
    );
}

#[test]
fn file_files() {
    let output = Exe::fixtures()
        .cmd
        .arg("--color=never")
        .arg("--include=../assets/include-files.input.txt")
        .output()
        .unwrap();
    assert_eq!(
        (
            u8v_to_utf8(&output.stdout),
            u8v_to_utf8(&output.stderr),
            output.status.success(),
        ),
        (
            include_str!("./expected-output/include-files.stdout.txt"),
            "ERROR: There are 1 unformatted files\n",
            false,
        ),
    );
}

#[test]
fn file_dirs() {
    let output = Exe::fixtures()
        .cmd
        .arg("--color=never")
        .arg("--include=../assets/include-dirs.input.txt")
        .output()
        .unwrap();
    assert_eq!(
        (
            u8v_to_utf8(&output.stdout),
            u8v_to_utf8(&output.stderr),
            output.status.success(),
        ),
        (
            include_str!("./expected-output/include-dirs.stdout.txt"),
            "ERROR: There are 5 unformatted files\n",
            false,
        ),
    );
}
