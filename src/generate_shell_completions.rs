use super::cli_opt::CliOpt;
use structopt_utilities::StructOptUtils;

pub fn main() {
    CliOpt::run_completion_generator("sane-fmt-generate-shell-completions", "sane-fmt")
}
