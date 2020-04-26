pub mod detail_level;

use structopt::StructOpt;
pub use detail_level::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "sane-fmt", rename_all = "kebab")]
pub struct CliOpt {
  /// File diff detail
  /// [possible values: count, name, diff]
  #[structopt(long, default_value = "diff")]
  details: DetailLevel,
}
