pub mod detail_level;
pub mod input_stream_address;
pub mod log_format;
pub mod when;

pub use detail_level::*;
pub use input_stream_address::*;
pub use log_format::*;
pub use when::*;

use std::{env::args, process::exit};
use structopt::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "sane-fmt", rename_all = "kebab")]
pub struct CliOpt {
    /// Reads unformatted code from standard input,
    /// prints formatted code to standard output, then exits.
    #[structopt(long)]
    pub stdio: bool,

    /// Whether to write or check
    #[structopt(long, short = "w")]
    pub write: bool,

    /// File diff detail
    #[structopt(long, default_value = "name", possible_values = &["count", "name", "diff"])]
    pub details: DetailLevel,

    /// Do not log passed filenames
    #[structopt(long)]
    pub hide_passed: bool,

    /// When to use terminal color
    #[structopt(long, default_value = "auto", possible_values = &["auto", "never", "always"])]
    pub color: When,

    /// Format of log messages
    #[structopt(long, default_value = "human", possible_values = &["human", "github-actions"])]
    pub log_format: LogFormat,

    /// Files whose contents contain paths to target files
    /// (`-` means stdin, other strings mean text file)
    #[structopt(long, short = "I")]
    pub include: Option<InputStreamAddress>,

    /// Files to process
    ///
    /// If none are provided, a default set of files will be assumed
    #[structopt(name = "files")]
    pub files: Vec<String>,
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
