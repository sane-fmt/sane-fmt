#![cfg(test)]
pub mod utils;
pub use utils::*;

use cargo_toml::*;
use package_json::*;

macro_rules! test_version {
    ($test_name:ident, $manifest_path:literal) => {
        #[test]
        fn $test_name() {
            let native = CargoManifest::load();
            let nodejs = NodeManifest::parse(include_str!($manifest_path));
            assert_eq!(nodejs.version, native.package.version);
        }
    };
}

test_version!(wasm32_wasi_version, "../nodejs/wasm32-wasi/package.json");
