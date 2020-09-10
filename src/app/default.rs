use super::super::{cli_opt::CliOpt, rules::build_fmt};
use super::App;
use structopt_utilities::StructOptUtils;

impl Default for App {
    fn default() -> Self {
        Self {
            opt: CliOpt::strict_from_args(),
            fmt: build_fmt(),
        }
    }
}
