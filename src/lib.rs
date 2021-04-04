pub use structopt::{self, clap};

pub mod app;
pub mod cli_opt;
pub mod export_json_config;
pub mod generate_shell_completions;
pub mod rules;

mod act;
mod cross_platform_path;
mod diff;
mod file_list;
mod term;

/// Initialize `app::App` with default values and runs it.
pub fn run() -> Result<(), String> {
    app::App::default().run()
}

/// The main program.
///
/// It calls [`run`], analyses the result:
/// * If it returns an `Ok`, exits with status `0`.
/// * If it returns an `Err`, prints the message to stdout and exits with status `1`.
pub fn main() -> ! {
    std::process::exit(if let Err(message) = run() {
        eprintln!("ERROR: {}", message);
        1
    } else {
        0
    })
}
