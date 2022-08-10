pub use clap;
pub use clap_utilities;

pub mod app;
pub mod cli_opt;
pub mod export_json_config;
pub mod generate_shell_completions;
pub mod pretty_error_message;
pub mod rules;

mod act;
mod cross_platform_path;
mod diff;
mod file_list;
mod term;

use std::process::ExitCode;

/// Initialize `app::App` with default values and runs it.
pub fn run() -> Result<(), String> {
    app::App::default().run()
}

/// The main program.
///
/// It calls [`run`], analyses the result:
/// * If it returns an `Ok`, returns [`ExitCode::SUCCESS`].
/// * If it returns an `Err`, prints the message to stdout and returns [`ExitCode::FAILURE`].
pub fn main() -> ExitCode {
    if let Err(message) = run() {
        eprint!("{}", pretty_error_message::PrettyErrorMessage(message));
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
