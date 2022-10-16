#![cfg(test)]
pub mod utils;
pub use utils::*;

use ansi_term::*;

#[test]
fn details_diff() {
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
            include_str!("./expected-output/github-actions-legacy/details-diff.stdout.txt"),
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
        .env_remove("GITHUB_OUTPUT")
        .output()
        .unwrap();

    assert_trimmed_str_eq(
        visualize_command_output(&output, &Style::new()).as_str(),
        visualize_fake_command_output(
            1,
            include_str!("./expected-output/github-actions-legacy/details-name.stdout.txt"),
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
        .env_remove("GITHUB_OUTPUT")
        .output()
        .unwrap();

    assert_trimmed_str_eq(
        visualize_command_output(&output, &Style::new()).as_str(),
        visualize_fake_command_output(
            1,
            include_str!("./expected-output/github-actions-legacy/details-count.stdout.txt"),
            include_str!("./expected-output/stderr.txt"),
            &Style::new(),
        )
        .as_str(),
    );
}
