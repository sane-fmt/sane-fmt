#![cfg(test)]
pub mod utils;
pub use utils::*;

#[test]
fn prints_formatted_code() {
    let unformatted = b"function hello () { return \"world\"; }";
    let formatted = format!(
        "{}\n{}\n{}\n",
        "function hello() {", "  return 'world'", "}",
    );

    let output = Exe::workspace().run_with_stdio(unformatted, &["--stdio"]);

    assert_eq!(
        (
            u8v_to_utf8(&output.stdout),
            u8v_to_utf8(&output.stderr),
            output.status.success(),
        ),
        (formatted.as_str(), "", true),
    );
}

#[test]
fn parse_failure() {
    let output = Exe::workspace().run_with_stdio(b"this is not a valid code", &["--stdio"]);

    assert_eq!(
        (u8v_to_utf8(&output.stdout), output.status.success()),
        ("", false),
        "stdout and status",
    );

    let stderr = u8v_to_utf8(&output.stderr);
    assert!(
        stderr.starts_with("Error: \"Failed to parse STDIN:"),
        "stderr: {}",
        stderr,
    );
}
