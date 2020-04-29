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

    // Identical in visible content
    assert_str_eq(
        u8v_to_utf8(&strip_ansi(&with_color).unwrap()),
        u8v_to_utf8(&without_color),
    );

    // But different in bytes
    assert_ne!(&with_color, &without_color);
}

#[test]
fn without_color() {
    let output = Exe::workspace()
        .mut_cmd(|cmd| {
            cmd.arg("--show-skipped")
                .arg("--details=diff")
                .arg("--color=never");
        })
        .cmd
        .output()
        .expect("spawn command without color");

    fn test(text: &Vec<u8>) {
        assert_str_eq(u8v_to_utf8(&strip_ansi(text).unwrap()), u8v_to_utf8(text));
    }

    test(&output.stdout);
    test(&output.stderr);
}
