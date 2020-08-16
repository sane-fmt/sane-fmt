pub mod app;
pub mod cli_opt;
pub mod export_json_config;
pub mod rules;

mod act;
mod cross_platform_path;
mod diff;
mod file_list;
mod term;

/// The main program.
///
/// It initializes `app::App` with default values and runs it.
pub fn main() -> Result<(), String> {
    app::App::default().run()
}
