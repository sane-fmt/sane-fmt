pub mod detail_level;

use structopt::StructOpt;
pub use detail_level::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "sane-fmt", rename_all = "kebab")]
pub struct CliOpt {
  /// Whether to write or check
  #[structopt(long, short = "w")]
  write: bool,

  /// File diff detail
  /// [possible values: count, name, diff]
  #[structopt(long, default_value = "diff")]
  details: DetailLevel,

  /// Glob pattern of paths to avoid
  #[structopt(long, short = "i", number_of_values = 1)]
  ignore: Vec<String>,

  /// Path to files or directories of files to format
  #[structopt(name = "FILES")]
  files: Vec<std::path::PathBuf>,
}
