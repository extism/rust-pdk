plugins:
	mkdir -p test
	cargo build --release --example count_vowels
	cargo build --release --example http
	cp target/wasm32-unknown-unknown/release/examples/count_vowels.wasm test/code.wasm
	cp target/wasm32-unknown-unknown/release/examples/http.wasm test/http.wasm