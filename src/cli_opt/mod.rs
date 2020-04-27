pub mod detail_level;

pub use detail_level::*;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "sane-fmt", rename_all = "kebab")]
pub struct CliOpt {
    /// Whether to write or check
    #[structopt(long, short = "w")]
    pub write: bool,

    /// File diff detail
    /// [possible values: count, name, diff]
    #[structopt(long, default_value = "name")]
    pub details: DetailLevel,

    /// Glob patterns of files to format
    ///
    /// If none are provided, a default set of patterns will be assumed
    #[structopt(name = "patterns")]
    pub patterns: Vec<String>,
}
