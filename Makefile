target/wasm32-wasi/release/sane-fmt.wasm:
	cargo wasi build --release

nodejs/wasm32-wasi/sane-fmt.wasm: target/wasm32-wasi/release/sane-fmt.wasm
	cp target/wasm32-wasi/release/sane-fmt.wasm nodejs/wasm32-wasi/sane-fmt.wasm
