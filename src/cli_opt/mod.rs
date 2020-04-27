pub mod detail_level;
pub mod when;

pub use detail_level::*;
pub use when::*;
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

    /// When to use terminal color
    /// [possible values: auto, never, always]
    #[structopt(long, default_value = "auto")]
    pub color: When,

    /// Glob patterns of files to format
    ///
    /// If none are provided, a default set of patterns will be assumed
    #[structopt(name = "patterns")]
    pub patterns: Vec<String>,
}
