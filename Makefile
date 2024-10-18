# TODO: Separate out run and build, store run logs in prove.log instead of cycles.txt
export RUST_LOG=info

# Rust benchmarks
merkle-rust = algorithms/merkle
merkle-proof-rust = algorithms/merkle_proof
nth-prime-rust = algorithms/nth_prime
keccak-rust = algorithms/keccak
poseidon-rust = algorithms/poseidon
bls-agg-rust = algorithms/bls-agg
ecdsa-verify-rust = algorithms/ecdsa_verify
bls-verify-rust = algorithms/bls-verify

all:
	@echo "Running all benchmarks..."
	make run-nth-prime
	make run-merklize
	make run-merkle-proof
	make run-keccak
	make run-poseidon
	make run-bls-aggregate
	make run-ecdsa-verify
	make run-bls-verify

run-merklize:
	@echo "Running Merkle benchmark..."
	cd ${merkle-rust}/native; cargo run --release -- --execute > cycles.txt
	cd ${merkle-rust}/native_sp1/script; SP1_PROVER=network cargo run --release > cycles.txt
	cd ${merkle-rust}/wasm; wasm-pack build
	cd ${merkle-rust}/wasm_sp1/script; SP1_PROVER=network cargo run --release > cycles.txt
	cd ${merkle-rust}/wasm_risc_zero/; RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run &> cycles.txt
	cd ${merkle-rust}/native_risc_zero; RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run &> cycles.txt
	cd ${merkle-rust}/delphinus; make build && ./test.sh > cycles.txt

run-nth-prime:
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

run-merkle-proof:
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

run-bls-aggregate:
	@echo "Running BLS aggregate benchmark..."
	cd ${bls-agg-rust}/native; cargo run --release -- --execute > cycles.txt
	cd ${bls-agg-rust}/native_sp1/script; cargo run --release -- --execute > cycles.txt
	cd $(bls-agg-rust)/native_sp1/script; SP1_PROVER=network SP1_PRIVATE_KEY=${SP1_PRIVATE_KEY} RUST_LOG=info cargo run --release -- --prove &> prove.log
	cd ${bls-agg-rust}/wasm; wasm-pack build
	cd ${bls-agg-rust}/wasm_sp1/script; cargo run --release -- --execute > cycles.txt
	cd ${bls-agg-rust}/wasm_risc_zero/; RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run &> cycles.txt
	cd ${bls-agg-rust}/native_risc_zero; RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run &> cycles.txt
	cd ${bls-agg-rust}/native_risc_zero; (time BONSAI_API_KEY=${BONSAI_API_KEY} BONSAI_API_URL=${BONSAI_API_URL} RUST_LOG="[executor]=info" RISC0_DEV_MODE=0 cargo run --release) &> prove.log

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
