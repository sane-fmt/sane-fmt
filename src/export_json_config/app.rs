use super::super::rules::build_cfg;
use super::{
    cfg::{DprintCfg, TypeScriptCfgWithSchema},
    cli_opt::{output_kind::OutputKind, CliOpt},
};
use clap::Parser;
use pipe_trait::*;
use serde_json::to_string_pretty as dump_json;

/// Application state.
pub struct App {
    /// Command-Line Options.
    pub opt: CliOpt,
}

impl Default for App {
    fn default() -> Self {
        App {
            opt: CliOpt::parse(),
        }
    }
}

impl App {
    /// Run the program based on application state.
    pub fn run(&self) {
        let CliOpt { kind, ref output } = self.opt;
        let cfg = build_cfg();

        let json = match kind {
            OutputKind::TypeScript => cfg.pipe(TypeScriptCfgWithSchema::from).pipe_ref(dump_json),
            OutputKind::Dprint => cfg.pipe(DprintCfg::from).pipe_ref(dump_json),
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
