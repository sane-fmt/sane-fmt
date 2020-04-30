#![cfg(test)]
pub mod utils;
pub use utils::*;

#[test]
fn version() {
    let output = Exe::workspace().cmd.arg("--version").output().unwrap();
    assert_str_eq(
        u8v_to_utf8(&output.stdout),
        format!("{} {}\n", NAME, VERSION).as_str(),
    );
    assert_eq!(output.status.success(), true);
}

#[test]
fn help() {
    let output = Exe::workspace().cmd.arg("--help").output().unwrap();

    assert_trimmed_str_eq(
        u8v_to_utf8(&output.stdout),
        format!(
            "{} {}\n\n{}",
            NAME,
            VERSION,
            include_str!("./expected-output/help.stdout.txt")
        )
        .as_str(),
    );
    assert_eq!(output.status.success(), true);
}

#[test]
fn unknown_flag() {
    let output = Exe::workspace()
        .cmd
        .arg("--completely-unknown-flag")
        .output()
        .unwrap();

    assert_str_eq(
        u8v_to_utf8(&output.stdout),
        include_str!("./expected-output/unknown-flag.stdout.txt"),
    );
    assert_eq!(output.status.success(), false);
}
