#![cfg(test)]
use copy_dir::copy_dir;
use difference::{Changeset, Difference};
use std::fmt::Write;
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    process::{Child as ChildProcess, Command},
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
        abs_copy_dir(fixtures(), temp_dir.join("fixtures"));
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
pub fn abs_copy_dir<Src, Dst>(source: Src, destination: Dst)
where
    Src: AsRef<Path>,
    Dst: AsRef<Path>,
{
    let errors: Vec<std::io::Error> = copy_dir(source, destination)
        .map_err(|error| format!("Failed to copy recursively: {}", error))
        .unwrap();
    if errors.len() != 0 {
        panic!(
            "Failed to copy recursively: {} errors: {:?}",
            errors.len(),
            errors
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

    let mut add_prefix = |text: String, prefix: &str| {
        for line in text.split("\n") {
            writeln!(&mut diff_text, "{}{}", prefix, line).expect("add a line to diff_text");
        }
    };

    for diff in change_set.diffs {
        match diff {
            Difference::Same(text) => add_prefix(text, "   "),
            Difference::Add(text) => add_prefix(text, "  +"),
            Difference::Rem(text) => add_prefix(text, "  -"),
        };
    }

    panic!("strings are not equal:\n{}", diff_text);
}
