use super::cli_opt::CliOpt;
use clap_utilities::CommandFactoryExtra;
use std::process::ExitCode;

pub fn main() -> ExitCode {
    CliOpt::run_completion_generator()
}
