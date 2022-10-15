#![cfg(test)]
pub mod utils;
pub use utils::*;

use ansi_term::*;

#[test]
fn details_diff_legacy() {
    let output = Exe::fixtures()
        .cmd
        .arg("--log-format=github-actions")
        .arg("--details=diff")
        .arg("--color=never")
        .env_remove("GITHUB_OUTPUT")
        .output()
        .unwrap();

    assert_trimmed_str_eq(
        visualize_command_output(&output, &Style::new()).as_str(),
        visualize_fake_command_output(
            1,
            include_str!("./expected-output/github-actions/details-diff-legacy.stdout.txt"),
            include_str!("./expected-output/stderr.txt"),
            &Style::new(),
        )
        .as_str(),
    );
}

#[test]
fn details_name_legacy() {
    let output = Exe::fixtures()
        .cmd
        .arg("--log-format=github-actions")
        .arg("--details=name")
        .arg("--color=never")
        .env_remove("GITHUB_OUTPUT")
        .output()
        .unwrap();

    assert_trimmed_str_eq(
        visualize_command_output(&output, &Style::new()).as_str(),
        visualize_fake_command_output(
            1,
            include_str!("./expected-output/github-actions/details-name-legacy.stdout.txt"),
            include_str!("./expected-output/stderr.txt"),
            &Style::new(),
        )
        .as_str(),
    );
}

#[test]
fn details_count_legacy() {
    let output = Exe::fixtures()
        .cmd
        .arg("--log-format=github-actions")
        .arg("--details=count")
        .arg("--color=never")
        .env_remove("GITHUB_OUTPUT")
        .output()
        .unwrap();

    assert_trimmed_str_eq(
        visualize_command_output(&output, &Style::new()).as_str(),
        visualize_fake_command_output(
            1,
            include_str!("./expected-output/github-actions/details-count-legacy.stdout.txt"),
            include_str!("./expected-output/stderr.txt"),
            &Style::new(),
        )
        .as_str(),
    );
}
