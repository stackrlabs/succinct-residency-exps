# TODO: Separate out run and build, store run logs in prove.log instead of cycles.txt
export RUST_LOG=info

all: rust-native rust-sp1 rust-wasm-sp1 rust-risc-zero rust-wasm-risc-zero go-wasm-sp1 rust-jolt

# Rust benchmarks
binary-rust = binary/rust
prime-rust = prime/rust
merkle-rust = merkle/rust
merkle-proof-rust = merkle_proof/rust
tsp-rust = tsp/rust
eth-verify-rust = eth_verify/rust
nth-prime-rust = nth_prime/rust
keccak-rust = keccak/rust
poseidon-rust = poseidon/rust
bls-agg-rust = bls-agg/rust
ecdsa-verify-rust = ecdsa_verify/rust
bls-verify-rust = bls-verify/rust

binary-native = binary/rust/native
prime-native = prime/rust/native
merkle-native = merkle/rust/native
tsp-native = tsp/rust/native
eth-verify-native = eth_verify/rust/native
bls-agg-native = bls-agg/rust/native
rust-native:
	@echo "Running Rust native benchmark..."
	cd ${binary-native}; cargo run --release -- --execute > cycles.txt
	cd ${prime-native}; cargo run --release -- --execute > cycles.txt
	cd ${merkle-native}; cargo run --release -- --execute > cycles.txt
	cd ${tsp-native}; cargo run --release -- --execute > cycles.txt
	cd ${eth-verify-native}; cargo run --release -- --execute > cycles.txt
	cd ${bls-agg-native}; cargo run --release -- --execute > cycles.txt
binary-sp1 = binary/rust/native_sp1/script
prime-sp1 = prime/rust/native_sp1/script
merkle-sp1 = merkle/rust/native_sp1/script
tsp-sp1 = tsp/rust/native_sp1/script
eth-verify-sp1 = eth_verify/rust/native_sp1/script
bls-agg-sp1 = bls-agg/rust/native_sp1/script
rust-sp1:
	@echo "Running Rust SP1 benchmark..."
	cd ${binary-sp1}; cargo run --release -- --execute > cycles.txt
	cd ${prime-sp1}; cargo run --release -- --execute > cycles.txt
	cd ${merkle-sp1}; cargo run --release -- --execute > cycles.txt
	cd ${tsp-sp1}; cargo run --release -- --execute > cycles.txt
	cd ${eth-verify-sp1}; cargo run --release -- --execute > cycles.txt
	cd ${bls-agg-sp1}; cargo run --release -- --execute > cycles.txt

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
	cd ${bls-agg-rust}/wasm; wasm-pack build
	cd ${bls-agg-rust}/wasm_sp1/script; cargo run --release -- --execute > cycles.txt

rust-risc-zero:
	@echo "Running Rust RISC Zero benchmark..."
	cd ${binary-rust}/native_risc_zero; RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run &> cycles.txt
	cd ${prime-rust}/native_risc_zero; RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run &> cycles.txt
	cd ${merkle-rust}/native_risc_zero; RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run &> cycles.txt
	cd ${tsp-rust}/native_risc_zero; RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run &> cycles.txt
	cd ${eth-verify-rust}/native_risc_zero; RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run &> cycles.txt
	cd ${bls-agg-rust}/native_risc_zero; RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run &> cycles.txt

rust-wasm-risc-zero:
	@echo "Running Rust wasm RISC Zero benchmark..."
	cd ${prime-rust}/wasm; wasm-pack build
	cd ${prime-rust}/wasm_risc_zero/; RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run --release &> cycles.txt
	cd ${binary-rust}/wasm; wasm-pack build
	cd ${binary-rust}/wasm_risc_zero/; RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run --release &> cycles.txt
	cd ${merkle-rust}/wasm; wasm-pack build
	cd ${merkle-rust}/wasm_risc_zero/; RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run --release &> cycles.txt
	cd ${tsp-rust}/wasm; wasm-pack build
	cd ${tsp-rust}/wasm_risc_zero/; RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run --release &> cycles.txt
	cd ${eth-verify-rust}/wasm; wasm-pack build
	cd ${eth-verify-rust}/wasm_risc_zero/; RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run --release &> cycles.txt
	cd ${bls-agg-rust}/wasm; wasm-pack build
	cd ${bls-agg-rust}/wasm_risc_zero/; RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run --release &> cycles.txt

rust-wasm-jolt:
	@echo "Running Rust Wasm Jolt benchmark..."
	cd ${prime-rust}/wasm; wasm-pack build
	cd ${prime-rust}/wasm_jolt/; RUST_LOG="[executor]=info" cargo run &> cycles.txt
	cd ${binary-rust}/wasm; wasm-pack build
	cd ${binary-rust}/wasm_jolt/; RUST_LOG="[executor]=info" cargo run &> cycles.txt
	cd ${merkle-rust}/wasm; wasm-pack build
	cd ${merkle-rust}/wasm_jolt/; RUST_LOG="[executor]=info" cargo run &> cycles.txt
	cd ${tsp-rust}/wasm; wasm-pack build
	cd ${tsp-rust}/wasm_jolt/; RUST_LOG="[executor]=info" cargo run &> cycles.txt
	cd ${eth-verify-rust}/wasm; wasm-pack build
	cd ${eth-verify-rust}/wasm_jolt/; RUST_LOG="[executor]=info" cargo run &> cycles.txt
	cd ${bls-agg-rust}/wasm; wasm-pack build
	cd ${bls-agg-rust}/wasm_jolt/; RUST_LOG="[executor]=info" cargo run &> cycles.txt

prove-risc-zero:
	@echo "Reading environment variables..."
	@echo "BONSAI_API_KEY: ${BONSAI_API_KEY}"
	@echo "BONSAI_API_URL: ${BONSAI_API_URL}"
	@echo "Proving RISC Zero benchmarks [merkle]..."
	cd ${merkle-rust}/native_risc_zero; (time BONSAI_API_KEY=${BONSAI_API_KEY} BONSAI_API_URL=${BONSAI_API_URL} RUST_LOG="[executor]=info" RISC0_DEV_MODE=0 cargo run --release) &> prove.log
	cd ${merkle-rust}/wasm; wasm-pack build
	cd ${merkle-rust}/wasm_risc_zero/; (time BONSAI_API_KEY=${BONSAI_API_KEY} BONSAI_API_URL=${BONSAI_API_URL} RUST_LOG="[executor]=info" RISC0_DEV_MODE=0 cargo run --release) &> prove.log
	@echo "Proving RISC Zero benchmarks [nth-prime]..."
	cd ${nth-prime-rust}/native_risc_zero; (time BONSAI_API_KEY=${BONSAI_API_KEY} BONSAI_API_URL=${BONSAI_API_URL} RUST_LOG="[executor]=info" RISC0_DEV_MODE=0 cargo run --release) &> prove.log
	cd ${nth-prime-rust}/wasm; wasm-pack build
	cd ${nth-prime-rust}/wasm_risc_zero/; (time BONSAI_API_KEY=${BONSAI_API_KEY} BONSAI_API_URL=${BONSAI_API_URL} RUST_LOG="[executor]=info" RISC0_DEV_MODE=0 cargo run --release) &> prove.log
	@echo "Proving RISC Zero benchmarks [keccak]..."
	cd ${keccak-rust}/native_risc_zero; (time BONSAI_API_KEY=${BONSAI_API_KEY} BONSAI_API_URL=${BONSAI_API_URL} RUST_LOG="[executor]=info" RISC0_DEV_MODE=0 cargo run --release) &> prove.log
	cd ${keccak-rust}/wasm; wasm-pack build
	cd ${keccak-rust}/wasm_risc_zero/; (time BONSAI_API_KEY=${BONSAI_API_KEY} BONSAI_API_URL=${BONSAI_API_URL} RUST_LOG="[executor]=info" RISC0_DEV_MODE=0 cargo run --release) &> prove.log
	@echo "Proving RISC Zero benchmarks [merkle_proof]..."
	cd ${merkle-proof-rust}/native_risc_zero; (time BONSAI_API_KEY=${BONSAI_API_KEY} BONSAI_API_URL=${BONSAI_API_URL} RUST_LOG="[executor]=info" RISC0_DEV_MODE=0 cargo run --release) &> prove.log
	cd ${merkle-proof-rust}/wasm; wasm-pack build
	cd ${merkle-proof-rust}/wasm_risc_zero/; (time BONSAI_API_KEY=${BONSAI_API_KEY} BONSAI_API_URL=${BONSAI_API_URL} RUST_LOG="[executor]=info" RISC0_DEV_MODE=0 cargo run --release) &> prove.log
	@echo "Proving RISC Zero benchmarks [poseidon]..."
	cd ${poseidon-rust}/native_risc_zero; (time BONSAI_API_KEY=${BONSAI_API_KEY} BONSAI_API_URL=${BONSAI_API_URL} RUST_LOG="[executor]=info" RISC0_DEV_MODE=0 cargo run --release) &> prove.log
	cd ${poseidon-rust}/wasm; wasm-pack build
	cd ${poseidon-rust}/wasm_risc_zero/; (time BONSAI_API_KEY=${BONSAI_API_KEY} BONSAI_API_URL=${BONSAI_API_URL} RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run --release) &> prove.log
	@echo "Proving RISC Zero benchmarks [bls-agg]..."
	cd ${bls-agg-rust}/native_risc_zero; (time BONSAI_API_KEY=${BONSAI_API_KEY} BONSAI_API_URL=${BONSAI_API_URL} RUST_LOG="[executor]=info" RISC0_DEV_MODE=0 cargo run --release) &> prove.log
	@echo "Proving RISC Zero benchmarks [ecdsa_verify]..."
	cd ${ecdsa-verify-rust}/native_risc_zero; (time BONSAI_API_KEY=${BONSAI_API_KEY} BONSAI_API_URL=${BONSAI_API_URL} RUST_LOG="[executor]=info" RISC0_DEV_MODE=0 cargo run --release) &> prove.log
	cd ${ecdsa-verify-rust}/wasm; wasm-pack build
	cd ${ecdsa-verify-rust}/wasm_risc_zero/; (time BONSAI_API_KEY=${BONSAI_API_KEY} BONSAI_API_URL=${BONSAI_API_URL} RUST_LOG="[executor]=info" RISC0_DEV_MODE=0 cargo run --release) &> prove.log

rust-jolt:
	@echo "Running Rust Jolt benchmark..."
	cd ${prime-rust}/native_jolt; cargo run --release > cycles.txt
	cd ${binary-rust}/native_jolt; cargo run --release > cycles.txt
	cd ${merkle-rust}/native_jolt; cargo run --release > cycles.txt
	cd ${tsp-rust}/native_jolt; cargo run --release > cycles.txt
	# cd ${eth-verify-rust}/native_jolt; cargo run --release > cycles.txt

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

nth-prime:
	@echo "Running nth-prime benchmark..."
	cd ${nth-prime-rust}/native; cargo run --release -- --execute > cycles.txt
	cd ${nth-prime-rust}/native_jolt; cargo run --release > cycles.txt
	cd ${nth-prime-rust}/native_risc_zero; RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run &> cycles.txt
	cd ${nth-prime-rust}/native_sp1/script; SP1_PROVER=network cargo run --release > cycles.txt
	cd ${nth-prime-rust}/wasm; wasm-pack build
	cd ${nth-prime-rust}/wasm_jolt; cargo run --release > cycles.txt
	cd ${nth-prime-rust}/wasm_risc_zero/; RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run &> cycles.txt
	cd ${nth-prime-rust}/wasm_sp1/script; SP1_PROVER=network cargo run --release > cycles.txt

run-keccak:
	@echo "Running keccak benchmark..."
	cd ${keccak-rust}/native; cargo run --release -- --execute > cycles.txt
	cd ${keccak-rust}/native_jolt; cargo run --release > cycles.txt
	cd ${keccak-rust}/native_risc_zero; RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run &> cycles.txt
	cd ${keccak-rust}/native_sp1/script; SP1_PROVER=network cargo run --release > cycles.txt
	cd ${keccak-rust}/wasm; wasm-pack build
	cd ${keccak-rust}/wasm_jolt; cargo run --release > cycles.txt
	cd ${keccak-rust}/wasm_risc_zero/; RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run &> cycles.txt
	cd ${keccak-rust}/wasm_sp1/script; SP1_PROVER=network cargo run --release > cycles.txt

merkle-proof:
	@echo "Running Merkle Proof Generation benchmark..."
	cd ${merkle-proof-rust}/native; cargo run --release -- --execute > cycles.txt
	cd ${merkle-proof-rust}/native_jolt; cargo run --release > cycles.txt
	cd ${merkle-proof-rust}/native_risc_zero; RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run &> cycles.txt
	cd ${merkle-proof-rust}/native_sp1/script; SP1_PROVER=network cargo run --release > cycles.txt
	cd ${merkle-proof-rust}/wasm; wasm-pack build
	cd ${merkle-proof-rust}/wasm_jolt; cargo run --release > cycles.txt
	cd ${merkle-proof-rust}/wasm_risc_zero/; RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run &> cycles.txt
	cd ${merkle-proof-rust}/wasm_sp1/script; SP1_PROVER=network cargo run --release > cycles.txt

run-poseidon:
	@echo "Running Poseidon Hash benchmark..."
	cd ${poseidon-rust}/native; cargo run --release -- --execute > cycles.txt
	cd ${poseidon-rust}/native_jolt; cargo run --release > cycles.txt
	cd ${poseidon-rust}/native_risc_zero; RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run &> cycles.txt
	cd ${poseidon-rust}/native_sp1/script; SP1_PROVER=network cargo run --release > cycles.txt
	cd ${poseidon-rust}/wasm; wasm-pack build
	cd ${poseidon-rust}/wasm_jolt; cargo run --release > cycles.txt
	cd ${poseidon-rust}/wasm_risc_zero/; RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run &> cycles.txt
	cd ${poseidon-rust}/wasm_sp1/script; SP1_PROVER=network cargo run --release > cycles.txt

bls-aggregate:
	@echo "Running BLS aggregate benchmark..."
	# cd ${bls-agg-rust}/native; cargo run --release -- --execute > cycles.txt
	# cd ${bls-agg-rust}/native_sp1/script; cargo run --release -- --execute > cycles.txt
	# cd $(bls-agg-rust)/native_sp1/script; SP1_PROVER=network SP1_PRIVATE_KEY=${SP1_PRIVATE_KEY} RUST_LOG=info cargo run --release -- --prove &> prove.log
	# cd ${bls-agg-rust}/wasm; wasm-pack build
	# cd ${bls-agg-rust}/wasm_sp1/script; cargo run --release -- --execute > cycles.txt
	# cd ${bls-agg-rust}/wasm_risc_zero/; RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run &> cycles.txt
	cd ${bls-agg-rust}/native_risc_zero; RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run &> cycles.txt
	# cd ${bls-agg-rust}/native_risc_zero; (time BONSAI_API_KEY=${BONSAI_API_KEY} BONSAI_API_URL=${BONSAI_API_URL} RUST_LOG="[executor]=info" RISC0_DEV_MODE=0 cargo run --release) &> prove.log

run-ecdsa-verify:
	@echo "Running ECDSA Verify benchmark..."
	cd ${ecdsa-verify-rust}/native; cargo run --release -- --execute > cycles.txt
	cd ${ecdsa-verify-rust}/native_jolt; cargo run --release > cycles.txt
	cd ${ecdsa-verify-rust}/native_risc_zero; RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run &> cycles.txt
	cd ${ecdsa-verify-rust}/native_sp1/script; SP1_PROVER=network cargo run --release > cycles.txt
	cd ${ecdsa-verify-rust}/wasm; wasm-pack build
	cd ${ecdsa-verify-rust}/wasm_jolt; cargo run --release > cycles.txt
	cd ${ecdsa-verify-rust}/wasm_risc_zero/; RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run &> cycles.txt
	cd ${ecdsa-verify-rust}/wasm_sp1/script; SP1_PROVER=network cargo run --release > cycles.txt

run-bls-verify:
	@echo "Running BLS verify benchmark..."
	cd ${bls-verify-rust}/native; cargo run --release -- --execute > cycles.txt
	cd ${bls-verify-rust}/native_sp1/script; cargo run --release -- --execute > cycles.txt
	cd ${bls-verify-rust}/native_sp1/script;SP1_PROVER=network SP1_PRIVATE_KEY=${SP1_PRIVATE_KEY} RUST_LOG=info cargo run --release -- --prove > prove.log
	cd ${bls-verify-rust}/wasm; wasm-pack build
	cd ${bls-verify-rust}/wasm_sp1/script; cargo run --release -- --execute > cycles.txt
	cd ${bls-verify-rust}/wasm_sp1/script;SP1_PROVER=network SP1_PRIVATE_KEY=${SP1_PRIVATE_KEY} RUST_LOG=info cargo run --release -- --prove > prove.log
	cd ${bls-verify-rust}/native_risc_zero; RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run --release &> cycles.txt
	cd ${bls-verify-rust}/native_risc_zero; (time BONSAI_API_KEY=${BONSAI_API_KEY} BONSAI_API_URL=${BONSAI_API_URL} RUST_LOG="[executor]=info" RISC0_DEV_MODE=0 cargo run --release) &> prove.log
	cd ${bls-verify-rust}/wasm_risc_zero/; RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run --release &> cycles.txt
	
