pub mod utils;
pub use utils::*;

use ansi_term::*;
use std::process::Command;

#[test]
fn wasm32_wasi_help() {
    let nodejs_output = Command::new("node")
        .current_dir(WORKSPACE)
        .arg(wasm32_wasi_bin())
        .arg("--help")
        .output()
        .unwrap();

    let native_output = Exe::workspace().cmd.arg("--help").output().unwrap();

    assert_str_eq(
        visualize_command_output(&nodejs_output, &Style::new()).as_str(),
        visualize_command_output(&native_output, &Style::new()).as_str(),
    );
}

#[test]
fn wasm32_wasi_details_name() {
    let nodejs_output = Command::new("node")
        .current_dir(WORKSPACE)
        .arg(wasm32_wasi_bin())
        .arg("--color=never")
        .arg("tests/fixtures")
        .output()
        .unwrap();

    let native_output = Exe::workspace()
        .cmd
        .arg("--color=never")
        .arg("tests/fixtures")
        .output()
        .unwrap();

    assert_str_eq(
        visualize_command_output(&nodejs_output, &Style::new()).as_str(),
        visualize_command_output(&native_output, &Style::new()).as_str(),
    );
}
