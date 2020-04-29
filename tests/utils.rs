#![cfg(test)]
use ansi_term::{Color, Style};
use difference::{Changeset, Difference};
use std::{
    ffi::OsStr,
    fmt::Write,
    path::{Path, PathBuf},
    process::{Child as ChildProcess, Command, Output as CommandOutput},
};
use tempfile as tmp;

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
        abs_copy_dir(
            &fixtures().to_string_lossy(),
            &temp_dir.join("fixtures").to_string_lossy(),
        );
        Self::new(&temp_dir)
    }

    /// Use a function to mutate cmd
    pub fn mut_cmd<F>(&mut self, mutate: F) -> &mut Self
    where
        F: FnOnce(&mut Command),
    {
        mutate(&mut self.cmd);
        self
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

    let CommandOutput {
        status,
        stdout,
        stderr,
    } = output;

    if !status.success() {
        let mut text = format!("Failed to copy {} to {}", &source, &destination);
        writeln!(text, "STATUS: {}\n", status.code().unwrap()).unwrap();
        writeln!(text, "STDOUT:\n{}\n", u8v_to_utf8(&stdout)).unwrap();
        writeln!(text, "STDERR:\n{}\n", u8v_to_utf8(&stderr)).unwrap();
        panic!(text);
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

    let mut add_prefix = |text: String, prefix: String| {
        for line in text.split("\n") {
            writeln!(&mut diff_text, "{}{}", prefix, line).expect("add a line to diff_text");
        }
    };

    let same_style = Style::new().dimmed();
    let add_style = Style::new().on(Color::Red);
    let rem_style = Style::new().on(Color::Green);

    fn paint(text: String, style: &Style) -> String {
        style.paint(text).to_string()
    }

    for diff in change_set.diffs {
        match diff {
            Difference::Same(text) => add_prefix(paint(text, &same_style), "   ".to_string()),
            Difference::Add(text) => add_prefix(
                paint(text, &add_style),
                paint("  +".to_string(), &add_style),
            ),
            Difference::Rem(text) => add_prefix(
                paint(text, &rem_style),
                paint("  -".to_string(), &rem_style),
            ),
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
