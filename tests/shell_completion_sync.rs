#![cfg(test)]
pub mod utils;
pub use utils::*;

use clap_utilities::clap_complete::Shell::*;
use clap_utilities::CommandFactoryExtra;
use sane_fmt::cli_opt::CliOpt;

macro_rules! test_comp_file {
    ($test_name:ident, $shell:expr, $output:literal) => {
        #[test]
        fn $test_name() {
            eprintln!(
                "test_comp_file!({test_name}, {shell}, {output:?});",
                test_name = stringify!($test_name),
                shell = stringify!($shell),
                output = $output,
            );
            let expected =
                CliOpt::get_completion_string("sane-fmt", $shell).expect("get completion string");
            let actual = include_str!($output);
            assert_str_eq(&expected, actual);
        }
    };
}

test_comp_file!(bash, Bash, "../exports/completion.bash");
test_comp_file!(fish, Fish, "../exports/completion.fish");
test_comp_file!(zsh, Zsh, "../exports/completion.zsh");
test_comp_file!(powershell, PowerShell, "../exports/completion.ps1");
test_comp_file!(elvish, Elvish, "../exports/completion.elv");
