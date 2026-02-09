plugins:
	mkdir -p test
	cargo build --release --example count_vowels
	cargo build --release --example http
	cargo build --release --example http_headers
	cargo build --release --example host_function
	cargo build --release --example host_function_host
	cargo build --release --example host_function_safe
	cp target/wasm32-unknown-unknown/release/examples/count_vowels.wasm test/code.wasm
	cp target/wasm32-unknown-unknown/release/examples/http.wasm test/http.wasm
	cp target/wasm32-unknown-unknown/release/examples/http_headers.wasm test/http_headers.wasm
	cp target/wasm32-unknown-unknown/release/examples/host_function.wasm test/host_function.wasm
	cp target/wasm32-unknown-unknown/release/examples/host_function_host.wasm test/host_function_host.wasm
	cp target/wasm32-unknown-unknown/release/examples/host_function_safe.wasm test/host_function_safe.wasm
