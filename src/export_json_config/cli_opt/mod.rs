pub mod output_kind;

use std::path::PathBuf;
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
