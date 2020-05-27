use super::super::{cli_opt::CliOpt, rules::build_fmt};
use super::App;

impl Default for App {
    fn default() -> Self {
        Self {
            opt: CliOpt::get(),
            fmt: build_fmt(),
        }
    }
}
