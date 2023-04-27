build : 
	cargo build --target wasm32-unknown-unknown --release
	cp target/wasm32-unknown-unknown/release/status_message.wasm ./deploy/status_message.wasm