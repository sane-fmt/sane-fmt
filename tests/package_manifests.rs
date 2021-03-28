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

test_version!(wasi_version, "../nodejs/wasm32-wasi/package.json");
test_version!(apple_version, "../nodejs/x86_64-apple-darwin/package.json");
test_version!(
    windows_gnu_version,
    "../nodejs/x86_64-pc-windows-gnu/package.json"
);
test_version!(
    windows_msvc_version,
    "../nodejs/x86_64-pc-windows-msvc/package.json"
);
test_version!(
    linux_gnu_version,
    "../nodejs/x86_64-unknown-linux-gnu/package.json"
);
test_version!(
    linux_musl_version,
    "../nodejs/x86_64-unknown-linux-musl/package.json"
);
