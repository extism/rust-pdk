count_vowels:
	cargo build --release --example count_vowels
	cp target/wasm32-unknown-unknown/release/examples/count_vowels.wasm code.wasm

http:
	cargo build --release --example http
	cp target/wasm32-unknown-unknown/release/examples/http.wasm code.wasm
