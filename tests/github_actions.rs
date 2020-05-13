#![cfg(test)]
pub mod utils;
use ansi_term::*;
pub use utils::*;

#[test]
fn details_diff() {
    let output = Exe::fixtures()
        .cmd
        .arg("--log-format=github-actions")
        .arg("--details=diff")
        .arg("--color=never")
        .output()
        .unwrap();

    assert_trimmed_str_eq(
        visualize_command_output(&output, &Style::new()).as_str(),
        visualize_fake_command_output(
            1,
            include_str!("./expected-output/github-actions/details-diff.stdout.txt"),
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
        .arg("--log-format=github-actions")
        .arg("--details=name")
        .arg("--color=never")
        .output()
        .unwrap();

    assert_trimmed_str_eq(
        visualize_command_output(&output, &Style::new()).as_str(),
        visualize_fake_command_output(
            1,
            include_str!("./expected-output/github-actions/details-name.stdout.txt"),
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
        .arg("--log-format=github-actions")
        .arg("--details=count")
        .arg("--color=never")
        .output()
        .unwrap();

    assert_trimmed_str_eq(
        visualize_command_output(&output, &Style::new()).as_str(),
        visualize_fake_command_output(
            1,
            include_str!("./expected-output/github-actions/details-count.stdout.txt"),
            include_str!("./expected-output/stderr.txt"),
            &Style::new(),
        )
        .as_str(),
    );
}

#[test]
fn log_scan() {
    let log_format_github_actions_color_always = Exe::fixtures()
        .cmd
        .arg("--log-format=github-actions")
        .arg("--color=always")
        .output()
        .unwrap();

    assert_trimmed_str_eq(
        encode_ansi_text(u8v_to_utf8(&log_format_github_actions_color_always.stderr)).as_str(),
        include_str!("./expected-output/github-actions/colored.stderr.txt"),
    )
}
