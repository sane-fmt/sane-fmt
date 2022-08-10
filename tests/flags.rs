#![cfg(test)]
pub mod utils;
pub use utils::*;

fn executable() -> String {
    if cfg!(windows) {
        format!("{NAME}.exe")
    } else {
        NAME.to_string()
    }
}

fn correct_snapshot(text: &str) -> String {
    if cfg!(windows) {
        let from = format!("{} [OPTIONS]", NAME);
        let to = format!("{} [OPTIONS]", executable());
        text.replace(from.as_str(), to.as_str())
    } else {
        text.to_string()
    }
}

#[test]
fn version() {
    let output = Exe::workspace().cmd.arg("--version").output().unwrap();
    assert_str_eq(
        u8v_to_utf8(&output.stdout),
        format!("{NAME} {VERSION}\n").as_str(),
    );
    assert!(output.status.success());
}

#[test]
fn help() {
    let output = Exe::workspace().cmd.arg("--help").output().unwrap();

    println!(
        "Output of `sane-fmt --help`:\n{}",
        u8v_to_utf8(&output.stdout),
    );

    assert_trimmed_str_eq(
        u8v_to_utf8(&output.stdout),
        format!(
            "{NAME} {VERSION}\n{DESCRIPTION}\n\n{rest}",
            rest = correct_snapshot(include_str!("./expected-output/help.stdout.txt")),
        )
        .as_str(),
    );
    assert!(output.status.success());
}
