#![cfg(test)]
pub mod utils;
pub use utils::*;

use std::ops::Not;

#[test]
fn write() {
    let Exe {
        cmd: mut check_cmd,
        wdir,
    } = Exe::temp_fixtures();
    check_cmd.arg("--details=name").arg("--color=never");

    let first_check = check_cmd.output().unwrap();
    assert_trimmed_str_eq(
        u8v_to_utf8(&first_check.stdout),
        include_str!("./expected-output/details-name.stdout.txt"),
    );
    assert_str_eq(
        u8v_to_utf8(&first_check.stderr),
        include_str!("./expected-output/stderr.txt"),
    );
    assert!(first_check.status.success().not());

    let write_output = Exe::new(&wdir)
        .cmd
        .arg("--details=name")
        .arg("--color=never")
        .arg("--write")
        .output()
        .unwrap();
    assert_trimmed_str_eq(
        u8v_to_utf8(&write_output.stdout),
        include_str!("./expected-output/details-name.stdout.txt"),
    );
    assert_str_eq(u8v_to_utf8(&write_output.stderr), "");
    assert!(write_output.status.success());

    let second_check = check_cmd.output().unwrap();
    assert_trimmed_str_eq(
        u8v_to_utf8(&second_check.stdout),
        include_str!("./expected-output/write-all-passed.stdout.txt"),
    );
    assert_str_eq(u8v_to_utf8(&second_check.stderr), "");
    assert!(second_check.status.success());
}
