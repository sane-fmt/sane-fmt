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

impl PkgInfo {
    fn load_cargo_manifest() -> Self {
        let Package {
            version,
            description,
        } = CargoManifest::load().package;
        PkgInfo {
            version,
            description,
        }
    }

    fn parse_node_manifest(text: &'static str) -> Self {
        let NodeManifest {
            version,
            description,
        } = NodeManifest::parse(text);
        PkgInfo {
            version,
            description,
        }
    }
}

macro_rules! test_case {
    ($test_name:ident, $manifest_path:literal) => {
        #[test]
        fn $test_name() {
            let native = PkgInfo::load_cargo_manifest();
            let nodejs = PkgInfo::parse_node_manifest(include_str!($manifest_path));
            assert_eq!(native, nodejs);
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
