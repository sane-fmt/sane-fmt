#![cfg(test)]
use ansi_term::{Color, Style};
use difference::{Changeset, Difference};
use fs_extra::dir::{copy as copy_dir, CopyOptions as DirCopyOptions};
use sane_fmt::rules::build_fmt;
use std::{
    ffi::OsStr,
    fmt::Write,
    fs::{create_dir, write as write_file},
    path::{Path, PathBuf, MAIN_SEPARATOR},
    process::{Child as ChildProcess, Command, Output as CommandOutput, Stdio},
};
use tempfile as tmp;

/// Version of the package
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Name of the package
pub const NAME: &str = env!("CARGO_PKG_NAME");

/// Description of the package
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

/// Path to main executable.
pub const EXE: &str = env!("CARGO_BIN_EXE_sane-fmt");

/// Path to manifest.
pub const WORKSPACE: &str = env!("CARGO_MANIFEST_DIR");

/// Prefix of temp dir
pub const TEMP_PREFIX: &str = "sane-fmt-";

/// Suffix of temp dir
pub const TEMP_SUFFIX: &str = ".test.wdir";

/// Get path to directory of fixtures
pub fn fixtures() -> PathBuf {
    Path::new(WORKSPACE).join("tests").join("fixtures")
}

/// Wrapper of main executable
pub struct Exe {
    pub cmd: Command,
    pub wdir: PathBuf,
}

impl Exe {
    /// Create a wrapper with specified working directory
    pub fn new<S: AsRef<OsStr> + ?Sized>(wdir_ref: &S) -> Self {
        let mut cmd = Command::new(EXE);
        let wdir = Path::new(wdir_ref).to_path_buf();
        cmd.current_dir(&wdir);
        Self { cmd, wdir }
    }

    /// Run the command
    pub fn run(&mut self) -> ChildProcess {
        self.cmd
            .spawn()
            .map_err(|error| format!("Failed to execute main command: {}", error))
            .unwrap()
    }

    /// Takes stdin, runs command,and returns `Output`
    pub fn run_with_stdio(
        &mut self,
        stdin: &[u8],
        args: impl IntoIterator<Item = impl AsRef<OsStr>>,
    ) -> CommandOutput {
        use std::io::Write;
        let mut child = self
            .cmd
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        child
            .stdin
            .as_mut()
            .expect("unwrap child's stdin")
            .write_all(stdin)
            .expect("write data to child's stdin");
        child.wait_with_output().expect("wait for child's output")
    }

    /// Use workspace directory as working directory
    pub fn workspace() -> Self {
        Exe::new(WORKSPACE)
    }

    /// Use fixtures directory as working directory
    pub fn fixtures() -> Self {
        Exe::new(&fixtures())
    }

    /// Use a temporary copy of workspace as working directory
    pub fn temp_workspace() -> Self {
        let temp_dir = tmp::Builder::new()
            .prefix(TEMP_PREFIX)
            .suffix(TEMP_SUFFIX)
            .tempdir()
            .unwrap()
            .into_path();
        let sub_dir = temp_dir.join("tests");
        create_dir(&sub_dir).expect("mkdir");
        abs_copy_dir(
            &fixtures().to_string_lossy(),
            &sub_dir.join("fixtures").to_string_lossy(),
        );
        Self::new(&temp_dir)
    }

    /// Use a temporary copy of fixtures as working directory
    pub fn temp_fixtures() -> Self {
        let temp_dir = tmp::Builder::new()
            .prefix(TEMP_PREFIX)
            .suffix(TEMP_SUFFIX)
            .tempdir()
            .unwrap()
            .into_path()
            .join("fixtures");
        abs_copy_dir(&fixtures().to_string_lossy(), &temp_dir.to_string_lossy());
        Self::new(&temp_dir)
    }

    /// Use a temporary directory of a temporary file as a working directory
    pub fn temp_file(file_name: &str, file_content: &str) -> Self {
        let temp_dir = tmp::Builder::new()
            .prefix(TEMP_PREFIX)
            .suffix(TEMP_SUFFIX)
            .tempdir()
            .unwrap()
            .into_path();
        let file_path = temp_dir.join(file_name);
        write_file(&file_path, file_content).expect("write file");
        Self::new(&temp_dir)
    }
}

/// Replace `/` and `\` in a string with MAIN_SEPARATOR
pub fn correct_path_str<Text: AsRef<str>>(text: Text) -> String {
    text.as_ref()
        .chars()
        .map(|ch| match ch {
            '/' | '\\' => MAIN_SEPARATOR,
            _ => ch,
        })
        .collect()
}

/// Copy directory recursively without room for errors
pub fn abs_copy_dir(source: &str, destination: &str) {
    let source = correct_path_str(source);
    let destination = correct_path_str(destination);

    let mut options = DirCopyOptions::new();
    options.overwrite = true;
    options.copy_inside = true;

    copy_dir(&source, &destination, &options)
        .unwrap_or_else(|_| panic!("copy from {} to {} recursively", &source, &destination));
}

/// Convert all newlines (be it LF or CRLF) to LF
pub fn normalize_line_ending(text: &str) -> String {
    text.lines().collect::<Vec<_>>().join("\n")
}

/// Assert two strings are equal.
/// If not, print diffs and panic.
pub fn assert_str_eq(a: &str, b: &str) {
    let a = normalize_line_ending(a);
    let a = a.as_str();
    let b = normalize_line_ending(b);
    let b = b.as_str();

    if a == b {
        return;
    }

    let change_set = Changeset::new(a, b, "\n");
    let mut diff_text = String::new();

    fn paint(text: &str, style: &Style) -> String {
        style.paint(text).to_string()
    }

    let mut make_lines = |text: String, prefix: &str, style: &Style| {
        for line in text.lines() {
            writeln!(diff_text, "{}{}", paint(prefix, style), paint(line, style))
                .expect("add a line to diff_text");
        }
    };

    let same_style = Style::new().dimmed();
    let add_style = Color::Green.into();
    let rem_style = Color::Red.into();

    for diff in change_set.diffs {
        match diff {
            Difference::Same(text) => make_lines(text, "   ", &same_style),
            Difference::Add(text) => make_lines(text, "  +", &add_style),
            Difference::Rem(text) => make_lines(text, "  -", &rem_style),
        };
    }

    panic!("strings are not equal:\n{}", diff_text);
}

/// Convert a vector of bytes to UTF-8 string
pub fn u8v_to_utf8(u8v: &[u8]) -> &str {
    std::str::from_utf8(u8v).expect("convert a vector of bytes to UTF-8 string")
}

/// Trim trailing whitespaces from every line of text and trailing newlines.
pub fn trim_trailing_whitespaces(text: &str) -> String {
    text.lines()
        .map(|line| line.trim_end())
        .collect::<Vec<_>>()
        .join("\n")
        .trim_end()
        .to_string()
}

/// Assert two strings are equal after being trimmed of trailing whitespaces.
/// If not, print diffs and panic.
pub fn assert_trimmed_str_eq(a: &str, b: &str) {
    assert_str_eq(
        trim_trailing_whitespaces(a).as_str(),
        trim_trailing_whitespaces(b).as_str(),
    );
}

/// Run a rule test
pub fn run_rule_test(
    test_name: &'static str,
    file_ext: &'static str,
    given_formatted: &str,
    given_unformatted: &[&str],
) {
    let file_name = PathBuf::from(format!("{name}.{ext}", name = test_name, ext = file_ext));
    let fmt = build_fmt();

    let actual_formatted = fmt
        .format_text(&file_name, given_formatted)
        .expect("format correctly styled code");
    assert_eq!(actual_formatted, None);

    for (index, unformatted) in given_unformatted.iter().enumerate() {
        eprintln!("unformatted[{}]", index);
        let formatted = fmt
            .format_text(&file_name, unformatted)
            .expect("format incorrectly styled code")
            .expect("get formatted code");
        assert_ne!(&formatted, *unformatted, "code style does not change");
        assert_str_eq(&formatted, given_formatted);
    }
}

/// Test a rule
#[macro_export]
macro_rules! test_rule {
    ($test_name:ident, $file_ext:expr, $formatted:expr, $unformatted:expr) => {
        #[test]
        fn $test_name() {
            let test_name = std::stringify!($test_name);
            run_rule_test(test_name, $file_ext, $formatted, $unformatted);
        }
    };
}

/// Define a block of text composed of multiple lines
#[macro_export]
macro_rules! text_block {
    () => ("");
    ($line:literal) => {
        concat!($line)
    };
    ($line:literal $($rest:literal)+) => {
        concat!($line, "\n", text_block!($($rest)+))
    };
}

/// Show status code, stdout, and stderr of a command in a pretty manner
pub fn visualize_command_output(output: &CommandOutput, title_style: &Style) -> String {
    visualize_fake_command_output(
        output.status.code().expect("get status code"),
        u8v_to_utf8(&output.stdout),
        u8v_to_utf8(&output.stderr),
        title_style,
    )
}

/// Show status code, stdout, and stderr of a command in a pretty manner
pub fn visualize_fake_command_output(
    status: i32,
    stdout: &str,
    stderr: &str,
    title_style: &Style,
) -> String {
    let mut result = format!("\n{} {}\n", title_style.paint("status"), status);
    let mut write_stream = |title, stream: &str| {
        writeln!(result, "{}", title_style.paint(title)).expect("write title");
        for line in stream.split('\n') {
            let line = line.replace('\r', "\r  "); // make sure "\r" does not delete indentation
            writeln!(result, "{}", line).expect("write line");
        }
    };
    write_stream("stdout", stdout);
    write_stream("stderr", stderr);
    result
}

/// Convert special characters to escaped form
pub fn encode_ansi_text(text: &str) -> String {
    text.chars()
        .map(|ch| match ch {
            '\x00' => "\\0".to_string(),
            '\x1B' => "\\e".to_string(),
            '\r' => "\\r".to_string(),
            '\\' => "\\".to_string(),
            _ => ch.to_string(),
        })
        .collect::<Vec<_>>()
        .join("")
}

pub mod package_json {
    use serde::{Deserialize, Serialize};

    /// Structure of package.json
    #[derive(Serialize, Deserialize)]
    pub struct NodeManifest {
        pub version: String,
        pub description: String,
    }

    impl NodeManifest {
        /// Parse a string
        pub fn parse<Text: AsRef<str>>(text: Text) -> Self {
            serde_json::from_str(text.as_ref()).expect("parse text as package.json")
        }
    }
}
