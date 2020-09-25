use super::super::rules::build_cfg;
use super::{
    cfg::DprintCfg,
    cli_opt::{output_kind::OutputKind, CliOpt},
};
use pipe_trait::*;
use serde_json::to_string_pretty as dump_json;
use structopt_utilities::StructOptUtils;

/// Application state.
pub struct App {
    /// Command-Line Options.
    pub opt: CliOpt,
}

impl Default for App {
    fn default() -> Self {
        App {
            opt: CliOpt::strict_from_args(),
        }
    }
}

impl App {
    /// Run the program based on application state.
    pub fn run(&self) {
        let CliOpt { kind, ref output } = self.opt;
        let cfg = build_cfg();

        let json = match kind {
            OutputKind::TypeScript => dump_json(&cfg),
            OutputKind::DprintRc => cfg.pipe(DprintCfg::from).pipe_ref(dump_json),
        }
        .expect("convert config to json");

        if let Some(filename) = output {
            let json = format!("{}\n", json);
            if let Err(error) = std::fs::write(filename, json) {
                eprintln!("✗ {}: {}", filename.to_string_lossy(), error);
            } else {
                eprintln!("💾 {}", filename.to_string_lossy());
            }
        } else {
            println!("{}", json);
        }
    }
}
