export RUST_LOG=info

all: rust-native rust-sp1 rust-wasm-sp1

binary-native = binary/rust/native
prime-native = prime/rust/native
merkle-native = merkle/rust/native
tsp-native = tsp/rust/native
eth-verify-native = eth_verify/rust/native
rust-native:
	@echo "Running Rust native benchmark..."
	cd ${binary-native}; cargo run --release -- --execute
	cd ${prime-native}; cargo run --release -- --execute
	cd ${merkle-native}; cargo run --release -- --execute
	cd ${tsp-native}; cargo run --release -- --execute
	cd ${eth-verify-native}; cargo run --release -- --execute 

binary-sp1 = binary/rust/native_sp1/script
prime-sp1 = prime/rust/native_sp1/script
merkle-sp1 = merkle/rust/native_sp1/script
tsp-sp1 = tsp/rust/native_sp1/script
eth-verify-sp1 = eth_verify/rust/native_sp1/script
rust-sp1:
	@echo "Running Rust SP1 benchmark..."
	cd ${binary-sp1}; cargo run --release -- --execute > cycles.txt
	cd ${prime-sp1}; cargo run --release -- --execute > cycles.txt
	cd ${merkle-sp1}; cargo run --release -- --execute > cycles.txt
	cd ${tsp-sp1}; cargo run --release -- --execute > cycles.txt
	cd ${eth-verify-sp1}; cargo run --release -- --execute > cycles.txt

binary-wasm = binary/rust
prime-wasm = prime/rust
merkle-wasm = merkle/rust
tsp-wasm = tsp/rust
eth-verify-wasm = eth_verify/rust
rust-wasm-sp1:
	@echo "Running Rust wasm SP1 benchmark..."
	cd ${binary-wasm}/wasm; wasm-pack build
	cd ${binary-wasm}/wasm_sp1/script; cargo run --release -- --execute > cycles.txt
	cd ${prime-wasm}/wasm; wasm-pack build
	cd ${prime-wasm}/wasm_sp1/script; cargo run --release -- --execute > cycles.txt
	cd ${merkle-wasm}/wasm; wasm-pack build
	cd ${merkle-wasm}/wasm_sp1/script; cargo run --release -- --execute > cycles.txt
	cd ${tsp-wasm}/wasm; wasm-pack build
	cd ${tsp-wasm}/wasm_sp1/script; cargo run --release -- --execute > cycles.txt
	cd ${eth-verify-wasm}/wasm; wasm-pack build
	cd ${eth-verify-wasm}/wasm_sp1/script; cargo run --release -- --execute > cycles.txt
