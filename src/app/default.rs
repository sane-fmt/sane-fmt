use super::super::{cli_opt::CliOpt, rules::build_fmt};
use super::App;
use clap::Parser;

impl Default for App {
    fn default() -> Self {
        Self {
            opt: CliOpt::parse(),
            fmt: build_fmt(),
        }
    }
}
