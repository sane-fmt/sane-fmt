pub mod app;
pub mod cli_opt;

/// The main program.
///
/// It initializes `app::App` with default values and runs it.
pub fn main() {
    app::App::default().run()
}
