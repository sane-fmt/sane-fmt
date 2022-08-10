pub mod output_kind;

use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(name = "sane-fmt-export-json-config", rename_all = "kebab")]
pub struct CliOpt {
    /// File to write to
    /// [default: stdout]
    #[clap(long, short = 'o')]
    pub output: Option<PathBuf>,

    /// Type of config
    #[clap(name = "TYPE", value_enum)]
    pub kind: output_kind::OutputKind,
}
