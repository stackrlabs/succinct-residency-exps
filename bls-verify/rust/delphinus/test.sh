#!/bin/bash

set -e
set -x

rm -rf output
mkdir output

ZKWASM_DIR="../../../../zkWasm" # needs pre-installed
ZKWASM_CLI=$ZKWASM_DIR/target/release/zkwasm-cli
ZKWASM_PARAMS=$ZKWASM_DIR/params

$ZKWASM_CLI --params $ZKWASM_PARAMS bls_aggregate setup --wasm ./pkg/output.wasm
$ZKWASM_CLI --params $ZKWASM_PARAMS bls_aggregate dry-run --wasm ./pkg/output.wasm --output ./output --private 2:i64
# time ($ZKWASM_CLI --params $ZKWASM_PARAMS merkle prove --wasm ./pkg/output.wasm --output ./output --private 5:i64)
# $ZKWASM_CLI --params $ZKWASM_PARAMS merkle verify --wasm ./pkg/output.wasm --output ./output
