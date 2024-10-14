#!/bin/bash

set -e
set -x

rm -rf output
mkdir output

# make sure that you've complied zkWasm with --features cuda (gives Halo2), --features continuation (always needed) and --features perf flag.
# Wasm is ~3x quicker than Halo2 version.
# Binary built using `cargo build --release --features continuation --features perf`

ZKWASM_DIR="../../../zkWasm" # needs pre-installed
ZKWASM_CLI=$ZKWASM_DIR/target/release/zkwasm-cli
ZKWASM_PARAMS=$ZKWASM_DIR/params

# it's universal for all the circuits
$ZKWASM_CLI --params $ZKWASM_PARAMS nth_prime setup --wasm ./pkg/output.wasm

$ZKWASM_CLI --params $ZKWASM_PARAMS nth_prime dry-run --wasm ./pkg/output.wasm --output ./output --private $1:i64
time ($ZKWASM_CLI --params $ZKWASM_PARAMS nth_prime prove --wasm ./pkg/output.wasm --output ./output --private $1:i64)
# $ZKWASM_CLI --params $ZKWASM_PARAMS nth_prime verify --wasm ./pkg/output.wasm --output ./output

# Batch the slices after this
