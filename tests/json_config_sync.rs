#![cfg(test)]
pub mod utils;
pub use utils::*;

use pipe_trait::*;
use sane_fmt::{
    export_json_config::cfg::{DprintCfg, TypeScriptCfg},
    rules::build_cfg,
};

macro_rules! test_json_config {
    ($test_name:ident, $kind:ident, $output:literal) => {
        #[test]
        fn $test_name() {
            let actual = include_str!($output);
            let expected = build_cfg()
                .pipe($kind::from)
                .pipe_ref(serde_json::to_string_pretty)
                .unwrap()
                .pipe(|json| format!("{}\n", json));
            assert_str_eq(&expected, actual);
        }
    };
}

test_json_config!(dprintrc, DprintCfg, "../exports/sane-fmt.dprintrc.json");
test_json_config!(
    typescript,
    TypeScriptCfg,
    "../exports/sane-fmt.typescript.json"
);
