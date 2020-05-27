pub mod act;
pub mod app;
pub mod cli_opt;
pub mod cross_platform_path;
pub mod diff;
pub mod file_list;
pub mod rules;
pub mod term;

/// The main function.
///
/// It initializes `app::App` with default values and runs it.
pub fn main() -> Result<(), String> {
    app::App::default().run()
}
