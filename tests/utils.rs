#![cfg(test)]
use ansi_term::{Color, Style};
use difference::{Changeset, Difference};
use std::{
    ffi::OsStr,
    fmt::Write,
    fs::{create_dir, write as write_file},
    path::{Path, PathBuf},
    process::{Child as ChildProcess, Command, Output as CommandOutput},
};
use tempfile as tmp;

/// Version of the package
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Name of the package
pub const NAME: &str = env!("CARGO_PKG_NAME");

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

    /// Use workspace directory as working directory
    pub fn workspace() -> Self {
        Exe::new(WORKSPACE)
    }

    /// Use a temporary directory as working directory
    pub fn temp() -> Self {
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

/// Copy directory recursively without room for errors
pub fn abs_copy_dir(source: &str, destination: &str) {
    // I have attempted to use libraries such as fs_extra::dir::copy and
    // copy_dir::copy_dir but none of them can handle symbolic link.
    // For this reason, I will just use the cp command.

    let output = Command::new("cp")
        .arg("--recursive")
        .arg("--no-dereference")
        .arg(source)
        .arg(destination)
        .output()
        .expect("spawn cp command");

    if !output.status.success() {
        panic!(
            "Failed to copy {} to {}\n{}",
            &source,
            &destination,
            visualize_command_output(&output, &Style::new()),
        );
    }
}

/// Assert two strings are equal.
/// If not, print diffs and panic.
pub fn assert_str_eq(a: &str, b: &str) {
    if a == b {
        return;
    }

    let change_set = Changeset::new(a, b, "\n");
    let mut diff_text = String::new();

    fn paint(text: &str, style: &Style) -> String {
        style.paint(text).to_string()
    }

    let mut make_lines = |text: String, prefix: &str, style: &Style| {
        for line in text.split("\n") {
            writeln!(
                diff_text,
                "{}{}",
                paint(prefix, &style),
                paint(line, &style)
            )
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
pub fn u8v_to_utf8(u8v: &Vec<u8>) -> &str {
    std::str::from_utf8(u8v).expect("convert a vector of bytes to UTF-8 string")
}

/// Trim trailing whitespaces from every line of text and trailing newlines.
pub fn trim_trailing_whitespaces(text: &str) -> String {
    text.split("\n")
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

/// Assert two strings have the same lines after being trimmed of trailing whitespaces.
/// If not, print diffs and panic.
pub fn assert_same_trimmed_lines(a: &str, b: &str) {
    fn sort_lines(text: &str) -> String {
        let trimmed = trim_trailing_whitespaces(text);
        let mut vec = trimmed.split("\n").collect::<Vec<_>>();
        vec.sort();
        vec.join("\n")
    }

    assert_str_eq(sort_lines(a).as_str(), sort_lines(b).as_str());
}

/// Run a rule test
pub fn run_rule_test(
    test_name: &'static str,
    file_ext: &'static str,
    formatted: &str,
    unformatted: &Vec<&str>,
) {
    let file_name = format!("{}.{}", test_name, file_ext);

    let output = |content| {
        Exe::temp_file(file_name.as_str(), content)
            .cmd
            .arg("--details=diff")
            .arg("--color=auto")
            .arg(file_name.as_str())
            .output()
            .unwrap()
    };

    let test = |assert: fn(bool) -> bool, content| {
        let out = output(content);
        if !assert(out.status.success()) {
            panic!(visualize_command_output(
                &out,
                &Style::new().bold().underline()
            ));
        }
    };

    test(|x| x, formatted);
    for content in unformatted {
        test(|x| !x, content);
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

/// Show status code, stdout, and stderr of a command in a pretty manner
pub fn visualize_command_output(output: &CommandOutput, title_style: &Style) -> String {
    let mut result = format!(
        "\n{} {}\n",
        title_style.paint("status"),
        output.status.code().expect("get status code")
    );
    let mut write_stream = |title, stream| {
        writeln!(result, "{}", title_style.paint(title)).expect("write title");
        for line in u8v_to_utf8(stream).split("\n") {
            let line = line.replace("\r", "\r  "); // make sure "\r" does not delete indentation
            writeln!(result, "{}", line).expect("write line");
        }
    };
    write_stream("stdout", &output.stdout);
    write_stream("stderr", &output.stderr);
    result
}
