use super::super::cli_opt::CliOpt as SaneFmtCliOpt;
use super::cli_opt::CliOpt;
use pipe_trait::*;
use std::{
    fs::File,
    io::{stdout, Write},
};
use structopt::*;

/// Application state.
pub struct App {
    /// Command-Line Options.
    pub opt: CliOpt,
}

impl Default for App {
    fn default() -> Self {
        App { opt: CliOpt::get() }
    }
}

impl App {
    /// Run the program based on application state.
    pub fn run(&self) {
        let CliOpt { output, bin, shell } = &self.opt;

        let mut buf: Box<dyn Write> = if let Some(file_name) = output {
            file_name.pipe(File::open).unwrap().pipe(Box::new)
        } else {
            Box::new(stdout())
        };

        SaneFmtCliOpt::clap().gen_completions_to(bin, *shell, &mut buf);
    }
}
