# TODO: Separate out run and build, store run logs in prove.log instead of cycles.txt
export RUST_LOG=info

all: rust-native rust-sp1 rust-wasm-sp1 rust-risc-zero rust-wasm-risc-zero go-wasm-sp1

binary-native = binary/rust/native
prime-native = prime/rust/native
merkle-native = merkle/rust/native
tsp-native = tsp/rust/native
eth-verify-native = eth_verify/rust/native
rust-native:
	@echo "Running Rust native benchmark..."
	cd ${binary-native}; cargo run --release -- --execute > cycles.txt
	cd ${prime-native}; cargo run --release -- --execute > cycles.txt
	cd ${merkle-native}; cargo run --release -- --execute > cycles.txt
	cd ${tsp-native}; cargo run --release -- --execute > cycles.txt
	cd ${eth-verify-native}; cargo run --release -- --execute > cycles.txt

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

binary-rust = binary/rust
prime-rust = prime/rust
merkle-rust = merkle/rust
tsp-rust = tsp/rust
eth-verify-rust = eth_verify/rust
rust-wasm-sp1:
	@echo "Running Rust wasm SP1 benchmark..."
	cd ${binary-rust}/wasm; wasm-pack build
	cd ${binary-rust}/wasm_sp1/script; cargo run --release -- --execute > cycles.txt
	cd ${prime-rust}/wasm; wasm-pack build
	cd ${prime-rust}/wasm_sp1/script; cargo run --release -- --execute > cycles.txt
	cd ${merkle-rust}/wasm; wasm-pack build
	cd ${merkle-rust}/wasm_sp1/script; cargo run --release -- --execute > cycles.txt
	cd ${tsp-rust}/wasm; wasm-pack build
	cd ${tsp-rust}/wasm_sp1/script; cargo run --release -- --execute > cycles.txt
	cd ${eth-verify-rust}/wasm; wasm-pack build
	cd ${eth-verify-rust}/wasm_sp1/script; cargo run --release -- --execute > cycles.txt

rust-risc-zero:
	@echo "Running Rust RISC Zero benchmark..."
	cd ${binary-rust}/native_risc_zero; RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run &> cycles.txt
	cd ${prime-rust}/native_risc_zero; RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run &> cycles.txt
	cd ${merkle-rust}/native_risc_zero; RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run &> cycles.txt
	cd ${tsp-rust}/native_risc_zero; RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run &> cycles.txt
	cd ${eth-verify-rust}/native_risc_zero; RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run &> cycles.txt

rust-wasm-risc-zero:
	@echo "Running Rust wasm RISC Zero benchmark..."
	cd ${prime-rust}/wasm; wasm-pack build
	cd ${prime-rust}/wasm_risc_zero/; RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run &> cycles.txt
	cd ${binary-rust}/wasm; wasm-pack build
	cd ${binary-rust}/wasm_risc_zero/; RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run &> cycles.txt
	cd ${merkle-rust}/wasm; wasm-pack build
	cd ${merkle-rust}/wasm_risc_zero/; RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run &> cycles.txt
	cd ${tsp-rust}/wasm; wasm-pack build
	cd ${tsp-rust}/wasm_risc_zero/; RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run &> cycles.txt
	cd ${eth-verify-rust}/wasm; wasm-pack build
	cd ${eth-verify-rust}/wasm_risc_zero/; RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run &> cycles.txt

# go benchmarks
binary-go = binary/go
go-wasm-sp1:
	@echo "Running Go wasm SP1 benchmark..."
	cd ${binary-go}/wasm; tinygo build -o main.wasm -target=wasm-unknown main.go
	cd ${binary-go}/wasm_sp1/script; cargo run --release -- --execute > cycles.txt

merklize:
	@echo "Running Merkle benchmark..."
	cd ${merkle-rust}/native; cargo run --release -- --execute > cycles.txt
	cd ${merkle-rust}/native_sp1/script; SP1_PROVER=network cargo run --release > cycles.txt
	cd ${merkle-rust}/wasm; wasm-pack build
	cd ${merkle-rust}/wasm_sp1/script; SP1_PROVER=network cargo run --release > cycles.txt
	cd ${merkle-rust}/wasm_risc_zero/; RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run &> cycles.txt
	cd ${merkle-rust}/native_risc_zero; RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run &> cycles.txt
	cd ${merkle-rust}/delphinus; make build && ./test.sh > cycles.txt
