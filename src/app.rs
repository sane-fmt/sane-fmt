pub mod default;
pub mod run;

use super::{cli_opt::CliOpt, rules::Fmt};

/// Application state.
pub struct App {
    /// Command-Line Options.
    pub opt: CliOpt,

    /// Code formatter.
    pub fmt: Fmt,
}
