pub mod output_kind;

use std::path::PathBuf;
use std::{env::args, process::exit};
use structopt::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "sane-fmt-export-json-config", rename_all = "kebab")]
pub struct CliOpt {
    /// File to write to
    /// [default: stdout]
    #[structopt(long, short = "o")]
    pub output: Option<PathBuf>,

    /// Type of config
    /// [possible values: typescript, dprintrc]
    #[structopt(name = "type")]
    pub kind: output_kind::OutputKind,
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
