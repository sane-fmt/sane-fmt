#![cfg(test)]
pub mod utils;
pub use utils::*;

fn executable() -> String {
    if cfg!(windows) {
        format!("{}.exe", NAME)
    } else {
        NAME.to_owned()
    }
}

fn correct_snapshot(text: &str) -> String {
    if cfg!(windows) {
        let from = format!("{} [FLAGS] [OPTIONS]", NAME);
        let to = format!("{} [FLAGS] [OPTIONS]", executable());
        text.replace(from.as_str(), to.as_str())
    } else {
        text.to_owned()
    }
}

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
            correct_snapshot(include_str!("./expected-output/help.stdout.txt"))
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
        correct_snapshot(include_str!("./expected-output/unknown-flag.stdout.txt")).as_str(),
    );
    assert_eq!(output.status.success(), false);
}
