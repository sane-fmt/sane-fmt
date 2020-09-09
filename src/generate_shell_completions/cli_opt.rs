use std::path::PathBuf;
use std::{env::args, process::exit};
use structopt::*;

pub use clap::Shell;

#[derive(StructOpt, Debug)]
#[structopt(name = "sane-fmt-generate-shell-completions", rename_all = "kebab")]
pub struct CliOpt {
    /// File to write to
    /// [default: stdout]
    #[structopt(long, short = "o")]
    pub output: Option<PathBuf>,

    /// Binary name
    #[structopt(long, default_value = "sane-fmt")]
    pub bin: String,

    /// Type of shell
    #[structopt(name = "shell", possible_values = &["bash", "fish", "zsh", "powershell", "elvish"])]
    pub shell: Shell,
}

impl CliOpt {
    /// Parse arguments from `env::args`.
    ///
    /// Unlike `StructOpt::from_args`, this function treat unknown flags as errors.
    pub fn get() -> Self {
        match Self::from_iter_safe(args()) as Result<Self, clap::Error> {
            Ok(value) => value,
            Err(clap::Error { kind, message, .. }) => match kind {
                clap::ErrorKind::HelpDisplayed | clap::ErrorKind::VersionDisplayed => {
                    println!("{}", message);
                    exit(0);
                }
                _ => {
                    println!("{}", message);
                    exit(1);
                }
            },
        }
    }
}
