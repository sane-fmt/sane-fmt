#![cfg(test)]
pub mod utils;
pub use utils::*;

use sane_fmt::{clap::Shell::*, cli_opt::CliOpt, structopt::StructOpt};

macro_rules! test_comp_file {
    ($test_name:ident, $shell:expr, $output:literal) => {
        #[test]
        fn $test_name() {
            let actual = include_str!($output);
            let mut expected = Vec::new();
            CliOpt::clap().gen_completions_to("sane-fmt", $shell, &mut expected);
            let expected = u8v_to_utf8(&expected);
            assert_str_eq(expected, actual);
        }
    };
}

test_comp_file!(bash, Bash, "../exports/completion.bash");
test_comp_file!(fish, Fish, "../exports/completion.fish");
test_comp_file!(zsh, Zsh, "../exports/completion.zsh");
test_comp_file!(powershell, PowerShell, "../exports/completion.ps1");
test_comp_file!(elvish, Elvish, "../exports/completion.elv");
