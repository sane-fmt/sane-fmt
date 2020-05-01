#![cfg(test)]
pub mod utils;
use ansi_term::*;
pub use utils::*;

#[test]
fn details_diff() {
    let output = Exe::workspace()
        .cmd
        .arg("--details=diff")
        .arg("--color=never")
        .output()
        .unwrap();

    assert_trimmed_str_eq(
        visualize_command_output(&output, &Style::new()).as_str(),
        visualize_fake_command_output(
            1,
            include_str!("./expected-output/details-diff.stdout.txt"),
            include_str!("./expected-output/stderr.txt"),
            &Style::new(),
        )
        .as_str(),
    );
}

#[test]
fn details_name() {
    let output = Exe::workspace()
        .cmd
        .arg("--details=name")
        .arg("--color=never")
        .output()
        .unwrap();

    assert_trimmed_str_eq(
        visualize_command_output(&output, &Style::new()).as_str(),
        visualize_fake_command_output(
            1,
            include_str!("./expected-output/details-name.stdout.txt"),
            include_str!("./expected-output/stderr.txt"),
            &Style::new(),
        )
        .as_str(),
    );
}

#[test]
fn details_count() {
    let output = Exe::workspace()
        .cmd
        .arg("--details=count")
        .arg("--color=never")
        .output()
        .unwrap();

    assert_trimmed_str_eq(
        visualize_command_output(&output, &Style::new()).as_str(),
        visualize_fake_command_output(
            1,
            "SUMMARY: total 11; changed 5; unchanged 6; skipped 0\n",
            include_str!("./expected-output/stderr.txt"),
            &Style::new(),
        )
        .as_str(),
    );
}

#[test]
fn colored() {
    let output = Exe::workspace()
        .cmd
        .arg("--show-skipped")
        .arg("--details=diff")
        .arg("--color=always")
        .output()
        .unwrap();

    assert_trimmed_str_eq(
        encode_ansi_text(u8v_to_utf8(&output.stdout)).as_str(),
        include_str!("./expected-output/colored.stdout.txt"),
    );
}

#[test]
fn correct_only() {
    let output = Exe::workspace()
        .cmd
        .arg("--show-skipped")
        .arg("--details=name")
        .arg("--color=never")
        .arg("tests/fixtures/correct/a.ts")
        .arg("tests/fixtures/correct/b.ts")
        .arg("tests/fixtures/correct/c.js")
        .output()
        .unwrap();
    assert_trimmed_str_eq(
        visualize_command_output(&output, &Style::new()).as_str(),
        visualize_fake_command_output(
            0,
            include_str!("./expected-output/correct-only.stdout.txt"),
            "",
            &Style::new(),
        )
        .as_str(),
    );
}

#[test]
fn some_are_skipped() {
    let output = Exe::workspace()
        .cmd
        .arg("--show-skipped")
        .arg("--details=name")
        .arg("--color=never")
        .arg("tests/fixtures/correct/a.ts")
        .arg("tests/fixtures/correct/a.ts")
        .arg("tests/fixtures/ignored")
        .arg("tests/fixtures/correct/b.ts")
        .arg("tests/fixtures/correct/b.ts")
        .arg("tests/fixtures/ignored/symlink.ts")
        .arg("tests/fixtures/correct/c.js")
        .arg("tests/fixtures/correct/c.js")
        .output()
        .unwrap();
    assert_trimmed_str_eq(
        visualize_command_output(&output, &Style::new()).as_str(),
        visualize_fake_command_output(
            0,
            include_str!("./expected-output/some-are-skipped.stdout.txt"),
            "",
            &Style::new(),
        )
        .as_str(),
    );
}

#[test]
fn colored_and_skipped() {
    let output = Exe::workspace()
        .cmd
        .arg("--show-skipped")
        .arg("--details=name")
        .arg("--color=always")
        .arg("tests/fixtures/ignored")
        .arg("tests/fixtures/ignored/symlink.ts")
        .output()
        .unwrap();
    assert_trimmed_str_eq(
        encode_ansi_text(u8v_to_utf8(&output.stdout)).as_str(),
        include_str!("./expected-output/colored-and-skipped.stdout.txt"),
    );
    assert_eq!(output.status.success(), true);
}
