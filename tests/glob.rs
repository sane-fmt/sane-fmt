#![cfg(test)]
pub mod utils;
pub use utils::*;

#[test]
fn abc_only() {
    let output = Exe::workspace()
        .cmd
        .arg("--show-skipped")
        .arg("--details=name")
        .arg("--color=never")
        .arg("**/{a,b,c}.{ts,js}")
        .output()
        .expect("spawn command without color");
    assert_trimmed_str_eq(
        u8v_to_utf8(&output.stdout),
        include_str!("./expected-output/abc-only.stdout.txt"),
    );
    assert_str_eq(
        u8v_to_utf8(&output.stderr),
        include_str!("./expected-output/abc-only.stderr.txt"),
    );
    assert_eq!(output.status.success(), false);
}
