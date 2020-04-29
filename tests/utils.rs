#![cfg(test)]
use std::{
    env::temp_dir,
    ffi::OsStr,
    fs::{copy, create_dir},
    path::{Path, PathBuf},
    process::{Child as ChildProcess, Command},
};

/// Path to main executable.
pub const EXE: &str = env!("CARGO_BIN_EXE_sane-fmt");

/// Path to manifest.
pub const WORKSPACE: &str = env!("CARGO_MANIFEST_DIR");

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
    fn new<S: AsRef<OsStr> + ?Sized>(wdir_ref: &S) -> Self {
        let mut cmd = Command::new(EXE);
        let wdir = Path::new(wdir_ref).to_path_buf();
        cmd.current_dir(&wdir);
        Self { cmd, wdir }
    }

    /// Run the command
    fn run(&mut self) -> ChildProcess {
        self.cmd
            .spawn()
            .map_err(|error| format!("Failed to execute main command: {}", error))
            .unwrap()
    }

    /// Use workspace directory as working directory
    fn workspace() -> Self {
        Exe::new(WORKSPACE)
    }

    /// Use a temporary directory as working directory
}
