#![cfg(test)]
pub mod utils;
use ansi_term::*;
pub use utils::*;

#[test]
fn details_diff() {
    let output = Exe::fixtures()
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
    let output = Exe::fixtures()
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
    let output = Exe::fixtures()
        .cmd
        .arg("--details=count")
        .arg("--color=never")
        .output()
        .unwrap();

    assert_trimmed_str_eq(
        visualize_command_output(&output, &Style::new()).as_str(),
        visualize_fake_command_output(
            1,
            "SUMMARY: total 11; changed 5; unchanged 6\n",
            include_str!("./expected-output/stderr.txt"),
            &Style::new(),
        )
        .as_str(),
    );
}

#[test]
fn colored() {
    let output = Exe::fixtures()
        .cmd
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
fn directory() {
    let output = Exe::workspace()
        .cmd
        .arg("--details=name")
        .arg("--color=never")
        .arg("tests/fixtures")
        .output()
        .unwrap();
    assert_trimmed_str_eq(
        visualize_command_output(&output, &Style::new()).as_str(),
        visualize_fake_command_output(
            1,
            include_str!("./expected-output/directory.stdout.txt"),
            include_str!("./expected-output/stderr.txt"),
            &Style::new(),
        )
        .as_str(),
    );
}

#[test]
fn some_correct_files_only() {
    let output = Exe::workspace()
        .cmd
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
            include_str!("./expected-output/some-correct-files-only.stdout.txt"),
            "",
            &Style::new(),
        )
        .as_str(),
    );
}

#[test]
fn correct_directory_only() {
    let output = Exe::workspace()
        .cmd
        .arg("--details=name")
        .arg("--color=never")
        .arg("tests/fixtures/correct")
        .output()
        .unwrap();
    assert_trimmed_str_eq(
        visualize_command_output(&output, &Style::new()).as_str(),
        visualize_fake_command_output(
            0,
            include_str!("./expected-output/correct-directory-only.stdout.txt"),
            "",
            &Style::new(),
        )
        .as_str(),
    );
}
