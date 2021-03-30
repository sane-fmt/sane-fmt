#![cfg(test)]
pub mod utils;
pub use utils::*;

use cargo_toml::*;
use package_json::*;

#[derive(Debug, PartialEq, Eq)]
struct PkgInfo {
    version: String,
    description: String,
}

macro_rules! test_case {
    ($test_name:ident, $manifest_path:literal) => {
        mod $test_name {
            use super::*;

            fn cargo_manifest() -> Package {
                CargoManifest::load().package
            }

            fn node_manifest() -> NodeManifest {
                NodeManifest::parse(include_str!($manifest_path))
            }

            #[test]
            fn version() {
                assert_eq!(node_manifest().version, cargo_manifest().version);
            }

            #[test]
            fn description() {
                assert_eq!(node_manifest().description, cargo_manifest().description);
            }
        }
    };
}

test_case!(wasi, "../nodejs/wasm32-wasi/package.json");
test_case!(apple, "../nodejs/x86_64-apple-darwin/package.json");
test_case!(windows_gnu, "../nodejs/x86_64-pc-windows-gnu/package.json");
test_case!(
    windows_msvc,
    "../nodejs/x86_64-pc-windows-msvc/package.json"
);
test_case!(linux_gnu, "../nodejs/x86_64-unknown-linux-gnu/package.json");
test_case!(
    linux_musl,
    "../nodejs/x86_64-unknown-linux-musl/package.json"
);
