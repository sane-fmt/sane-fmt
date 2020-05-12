#![cfg(test)]
pub mod utils;
pub use utils::*;

macro_rules! test_readme {
    ($test_name:ident, $filename:literal) => {
        #[test]
        fn $test_name() {
            let a = include_str!("../README.md");
            let b = include_str!($filename);
            assert_str_eq(a, b);
        }
    };
}

test_readme!(wasm32_wasi, "../nodejs/wasm32-wasi/README.md");
test_readme!(apple_darwin, "../nodejs/x86_64-apple-darwin/README.md");
test_readme!(windows_gnu, "../nodejs/x86_64-pc-windows-gnu/README.md");
test_readme!(windows_msvc, "../nodejs/x86_64-pc-windows-msvc/README.md");
test_readme!(linux_gnu, "../nodejs/x86_64-unknown-linux-gnu/README.md");
test_readme!(linux_musl, "../nodejs/x86_64-unknown-linux-musl/README.md");
