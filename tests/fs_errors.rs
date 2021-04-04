#![cfg(test)]
pub mod utils;
pub use utils::*;

#[test]
fn not_exist() {
    let output = Exe::fixtures()
        .cmd
        .arg("--color=never")
        .arg("path that does not exist")
        .output()
        .unwrap();

    #[cfg(unix)]
    let expected_stderr =
        "ERROR: path that does not exist: No such file or directory (os error 2)\n";
    #[cfg(windows)]
    let expected_stderr = "ERROR: path that does not exist: The system cannot find the file specified. (os error 2)\n";

    assert_eq!(
        (
            u8v_to_utf8(&output.stdout),
            u8v_to_utf8(&output.stderr),
            output.status.success(),
        ),
        ("", expected_stderr, false),
    );
}
