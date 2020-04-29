#![cfg(test)]
pub mod utils;
pub use utils::*;

use strip_ansi_escapes::strip as strip_ansi;

#[test]
fn color_match_non_color() {
    let with_color = Exe::workspace()
        .mut_cmd(|cmd| {
            cmd.arg("--show-skipped")
                .arg("--details=diff")
                .arg("--color=always");
        })
        .cmd
        .output()
        .expect("spawn command with color")
        .stdout;
    let without_color = Exe::workspace()
        .mut_cmd(|cmd| {
            cmd.arg("--show-skipped")
                .arg("--details=diff")
                .arg("--color=never");
        })
        .cmd
        .output()
        .expect("spawn command without color")
        .stdout;
    assert_eq!(strip_ansi(with_color).unwrap(), without_color);
}
