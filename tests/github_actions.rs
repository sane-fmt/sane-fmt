#![cfg(test)]
pub mod utils;

pub use utils::*;

use ansi_term::*;
use std::fs::read_to_string;
use tempfile::NamedTempFile;

fn gh_files() -> (NamedTempFile, NamedTempFile) {
    let gh_output_file = tempfile::Builder::new()
        .prefix("github-output")
        .suffix(".txt")
        .tempfile()
        .expect("create temporary file for GITHUB_OUTPUT");
    let gh_sum_file = tempfile::Builder::new()
        .prefix("github-step-summary")
        .suffix(".md")
        .tempfile()
        .expect("create temporary file for GITHUB_STEP_SUMMARY");
    (gh_output_file, gh_sum_file)
}

#[test]
fn details_diff() {
    let (gh_output_file, gh_sum_file) = gh_files();

    let output = Exe::fixtures()
        .cmd
        .arg("--log-format=github-actions")
        .arg("--details=diff")
        .arg("--color=never")
        .env("GITHUB_OUTPUT", gh_output_file.path())
        .env("GITHUB_STEP_SUMMARY", gh_sum_file.path())
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

    assert_trimmed_str_eq(
        &read_to_string(gh_output_file.path()).expect("read GITHUB_OUTPUT file"),
        include_str!("./expected-output/github-actions/github-output.txt"),
    );

    assert_trimmed_str_eq(
        &read_to_string(gh_sum_file.path()).expect("read GITHUB_STEP_SUMMARY file"),
        include_str!("./expected-output/github-actions/github-step-summary.md"),
    );
}

#[test]
fn details_name() {
    let (gh_output_file, gh_sum_file) = gh_files();

    let output = Exe::fixtures()
        .cmd
        .arg("--log-format=github-actions")
        .arg("--details=name")
        .arg("--color=never")
        .env("GITHUB_OUTPUT", gh_output_file.path())
        .env("GITHUB_STEP_SUMMARY", gh_sum_file.path())
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

    assert_trimmed_str_eq(
        &read_to_string(gh_output_file.path()).expect("read GITHUB_OUTPUT file"),
        include_str!("./expected-output/github-actions/github-output.txt"),
    );

    assert_trimmed_str_eq(
        &read_to_string(gh_sum_file.path()).expect("read GITHUB_STEP_SUMMARY file"),
        include_str!("./expected-output/github-actions/github-step-summary.md"),
    );
}

#[test]
fn details_count() {
    let (gh_output_file, gh_sum_file) = gh_files();

    let output = Exe::fixtures()
        .cmd
        .arg("--log-format=github-actions")
        .arg("--details=count")
        .arg("--color=never")
        .env("GITHUB_OUTPUT", gh_output_file.path())
        .env("GITHUB_STEP_SUMMARY", gh_sum_file.path())
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

    assert_trimmed_str_eq(
        &read_to_string(gh_output_file.path()).expect("read GITHUB_OUTPUT file"),
        include_str!("./expected-output/github-actions/github-output.txt"),
    );

    assert_trimmed_str_eq(
        &read_to_string(gh_sum_file.path()).expect("read GITHUB_STEP_SUMMARY file"),
        include_str!("./expected-output/github-actions/github-step-summary.md"),
    );
}
