#![cfg(test)]
pub mod utils;
pub use utils::*;

#[test]
fn prints_formatted_code() {
    let unformatted = b"function hello () { return \"world\"; }";
    let formatted = text_block! {
        "function hello() {"
        "  return 'world'"
        "}"
        ""
    };

    let output = Exe::workspace().run_with_stdio(unformatted, &["--stdio"]);

    assert_eq!(
        (
            u8v_to_utf8(&output.stdout),
            u8v_to_utf8(&output.stderr),
            output.status.success(),
        ),
        (formatted, "", true),
    );
}

#[test]
fn parse_failure() {
    let output = Exe::workspace().run_with_stdio(b"const invalid_code == invalid ;)", &["--stdio"]);

    assert_eq!(
        (u8v_to_utf8(&output.stdout), output.status.success()),
        ("", false),
        "stdout and status",
    );

    let stderr = u8v_to_utf8(&output.stderr);

    assert_str_eq(
        stderr,
        text_block! {
            "ERROR:"
            "  Line 1, column 20: Expected a semicolon"
            "  "
            "    const invalid_code == invalid ;"
            "                       ~~"
            "  "
            "  Line 1, column 20: Expression expected"
            "  "
            "    const invalid_code == invalid ;"
            "                       ~~"
        },
    );
}
