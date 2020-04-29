#![cfg(test)]
pub mod utils;
pub use utils::*;

#[test]
fn show_skipped_details_diff() {
    let output = Exe::workspace()
        .mut_cmd(|cmd| {
            cmd.arg("--show-skipped")
                .arg("--details=diff")
                .arg("--color=never");
        })
        .cmd
        .output()
        .expect("spawn command without color");

    assert_trimmed_str_eq(
        u8v_to_utf8(&output.stdout),
        include_str!("./expected-output/show-skipped-details-diff.stdout.txt"),
    );
    assert_trimmed_str_eq(
        u8v_to_utf8(&output.stderr),
        include_str!("./expected-output/stderr.txt"),
    );
    assert_eq!(output.status.success(), false);
}

#[test]
fn show_skipped_details_name() {
    let output = Exe::workspace()
        .mut_cmd(|cmd| {
            cmd.arg("--show-skipped")
                .arg("--details=name")
                .arg("--color=never");
        })
        .cmd
        .output()
        .expect("spawn command without color");

    assert_trimmed_str_eq(
        u8v_to_utf8(&output.stdout),
        include_str!("./expected-output/show-skipped-details-name.stdout.txt"),
    );
    assert_trimmed_str_eq(
        u8v_to_utf8(&output.stderr),
        include_str!("./expected-output/stderr.txt"),
    );
    assert_eq!(output.status.success(), false);
}

#[test]
fn hide_skipped_details_diff() {
    let output = Exe::workspace()
        .mut_cmd(|cmd| {
            cmd.arg("--details=diff").arg("--color=never");
        })
        .cmd
        .output()
        .expect("spawn command without color");

    assert_trimmed_str_eq(
        u8v_to_utf8(&output.stdout),
        include_str!("./expected-output/hide-skipped-details-diff.stdout.txt"),
    );
    assert_trimmed_str_eq(
        u8v_to_utf8(&output.stderr),
        include_str!("./expected-output/stderr.txt"),
    );
    assert_eq!(output.status.success(), false);
}

#[test]
fn hide_skipped_details_name() {
    let output = Exe::workspace()
        .mut_cmd(|cmd| {
            cmd.arg("--details=name").arg("--color=never");
        })
        .cmd
        .output()
        .expect("spawn command without color");

    assert_trimmed_str_eq(
        u8v_to_utf8(&output.stdout),
        include_str!("./expected-output/hide-skipped-details-name.stdout.txt"),
    );
    assert_trimmed_str_eq(
        u8v_to_utf8(&output.stderr),
        include_str!("./expected-output/stderr.txt"),
    );
    assert_eq!(output.status.success(), false);
}

#[test]
fn details_count() {
    let output = Exe::workspace()
        .mut_cmd(|cmd| {
            cmd.arg("--details=count").arg("--color=never");
        })
        .cmd
        .output()
        .expect("spawn command without color");

    assert_str_eq(
        u8v_to_utf8(&output.stdout),
        "SUMMARY: scanned 9; found 4; skipped 3\n",
    );
    assert_trimmed_str_eq(
        u8v_to_utf8(&output.stderr),
        include_str!("./expected-output/stderr.txt"),
    );
    assert_eq!(output.status.success(), false);
}

#[test]
fn colored() {
    let output = Exe::workspace()
        .mut_cmd(|cmd| {
            cmd.arg("--show-skipped")
                .arg("--details=diff")
                .arg("--color=always");
        })
        .cmd
        .output()
        .expect("spawn command without color");

    assert_trimmed_str_eq(
        u8v_to_utf8(&output.stdout)
            .split("")
            .map(|ch| match ch {
                "\x00" => "\\0",
                "\x1B" => "\\e",
                "\\" => "\\",
                _ => ch,
            })
            .collect::<Vec<_>>()
            .join("")
            .as_str(),
        include_str!("./expected-output/colored.stdout.txt"),
    );
}
