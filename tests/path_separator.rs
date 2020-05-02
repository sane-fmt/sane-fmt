#![cfg(test)]
pub mod utils;
use ansi_term::*;
pub use utils::*;

#[test]
fn unix_style_only() {
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
fn windows_style_only() {
    let output = Exe::workspace()
        .cmd
        .arg("--show-skipped")
        .arg("--details=name")
        .arg("--color=never")
        .arg(r"tests\fixtures\correct\a.ts")
        .arg(r"tests\fixtures\correct\b.ts")
        .arg(r"tests\fixtures\correct\c.js")
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
fn both_styles() {
    let output = Exe::workspace()
        .cmd
        .arg("--show-skipped")
        .arg("--details=name")
        .arg("--color=never")
        .arg(r"tests/fixtures/correct/a.ts")
        .arg(r"tests\fixtures/correct\b.ts")
        .arg(r"tests\fixtures\correct\c.js")
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
