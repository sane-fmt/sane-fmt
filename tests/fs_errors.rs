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

    assert_eq!(
        (
            u8v_to_utf8(&output.stdout),
            u8v_to_utf8(&output.stderr),
            output.status.success(),
        ),
        (
            "",
            "Error: \"path that does not exist: No such file or directory (os error 2)\"\n",
            false,
        ),
    );
}
