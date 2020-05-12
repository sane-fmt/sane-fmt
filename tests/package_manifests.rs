#![cfg(test)]
pub mod utils;
pub use utils::*;

use cargo_toml::*;
use package_json::*;

#[test]
fn wasm32_wasi_version() {
    let native = CargoManifest::load();
    let nodejs = NodeManifest::parse(include_str!("../nodejs/wasm32-wasi/package.json"));
    assert_eq!(nodejs.version, native.package.version);
}
