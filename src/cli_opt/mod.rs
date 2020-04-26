pub mod detail_level;

use structopt::StructOpt;
pub use detail_level::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "sane-fmt", rename_all = "kebab")]
pub struct CliOpt {
  /// Whether to write or check
  #[structopt(long, short = "w")]
  pub write: bool,

  /// File diff detail
  /// [possible values: count, name, diff]
  #[structopt(long, default_value = "diff")]
  pub details: DetailLevel,

  /// Glob patterns of files to format
  #[structopt(name = "patterns")]
  pub patterns: Vec<String>,
}
