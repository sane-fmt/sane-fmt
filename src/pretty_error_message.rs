use std::fmt::{Display, Error, Formatter};

/// A wrapper around a string of error message to print beautifully.
///
/// **NOTE:** It ends with a single trailing newline regardless of the how
/// many trailing newlines that the error message has.
pub struct PrettyErrorMessage<ErrorMessage: AsRef<str>>(pub ErrorMessage);

impl<ErrorMessage: AsRef<str>> Display for PrettyErrorMessage<ErrorMessage> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error> {
        let error_message = self.0.as_ref().trim_end_matches('\n');
        let lines: Vec<_> = error_message.lines().collect();

        if lines.len() <= 1 {
            return writeln!(formatter, "ERROR: {}", error_message);
        }

        writeln!(formatter, "ERROR:")?;
        for line in lines {
            writeln!(formatter, "    {}", line)?;
        }
        Ok(())
    }
}

#[test]
fn single_line_no_trailing_newline() {
    let actual = format!("{}", PrettyErrorMessage("Something goes wrong."));
    let expected = "ERROR: Something goes wrong.\n";
    assert_eq!(actual, expected);
}

#[test]
fn single_line_single_trailing_newline() {
    let actual = format!("{}", PrettyErrorMessage("Something goes wrong.\n"));
    let expected = "ERROR: Something goes wrong.\n";
    assert_eq!(actual, expected);
}

#[test]
fn single_line_multiple_trailing_newlines() {
    let actual = format!("{}", PrettyErrorMessage("Something goes wrong.\n\n\n"));
    let expected = "ERROR: Something goes wrong.\n";
    assert_eq!(actual, expected);
}

#[test]
fn multiple_lines() {
    let input_error_message = concat! {
        "Something goes wrong:\n",
        "\n",
        "const x = 123\n",
        "      ^ this name is too short\n",
        "\n",
        "Something goes wrong:\n",
        "\n",
        "var foo = 456\n",
        "^^^ var? This my modern JavaScript?\n",
        "\n",
        "\n",
        "\n",
    };

    let actual = format!("{}", PrettyErrorMessage(input_error_message));

    let expected = concat! {
        "ERROR:\n",
        "    Something goes wrong:\n",
        "    \n",
        "    const x = 123\n",
        "          ^ this name is too short\n",
        "    \n",
        "    Something goes wrong:\n",
        "    \n",
        "    var foo = 456\n",
        "    ^^^ var? This my modern JavaScript?\n",
    };

    assert_eq!(&actual, expected, "\nACTUAL:\n{}", &actual);
}
