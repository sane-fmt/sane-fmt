#![cfg(test)]
pub mod utils;

pub use utils::*;

use ansi_term::*;
use std::{fs::read_to_string, io::Write};

fn gh_output_file() -> tempfile::NamedTempFile {
    let mut file = tempfile::Builder::new()
        .prefix("github-output")
        .suffix(".txt")
        .tempfile()
        .expect("create temporary file for GITHUB_OUTPUT");
    writeln!(file, "EXISTING_VALUE=Something else").unwrap();
    file.flush().unwrap();
    file
}

#[test]
fn details_diff() {
    let gh_output_file = gh_output_file();

    let output = Exe::fixtures()
        .cmd
        .arg("--log-format=github-actions")
        .arg("--details=diff")
        .arg("--color=never")
        .env("GITHUB_OUTPUT", gh_output_file.path())
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
}

#[test]
fn details_name() {
    let gh_output_file = gh_output_file();

    let output = Exe::fixtures()
        .cmd
        .arg("--log-format=github-actions")
        .arg("--details=name")
        .arg("--color=never")
        .env("GITHUB_OUTPUT", gh_output_file.path())
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
}

#[test]
fn details_count() {
    let gh_output_file = gh_output_file();

    let output = Exe::fixtures()
        .cmd
        .arg("--log-format=github-actions")
        .arg("--details=count")
        .arg("--color=never")
        .env("GITHUB_OUTPUT", gh_output_file.path())
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
}
