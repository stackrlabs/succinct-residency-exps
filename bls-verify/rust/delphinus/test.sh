#!/bin/bash

set -e
set -x

rm -rf output
mkdir output

ZKWASM_DIR="../../../zkWasm" # needs pre-installed
ZKWASM_CLI=$ZKWASM_DIR/target/release/zkwasm-cli
ZKWASM_PARAMS=$ZKWASM_DIR/params

make build

$ZKWASM_CLI --params $ZKWASM_PARAMS bls_verify setup --wasm ./pkg/output.wasm
$ZKWASM_CLI --params $ZKWASM_PARAMS bls_verify dry-run --wasm ./pkg/output.wasm --output ./output --private $1:i64
# time ($ZKWASM_CLI --params $ZKWASM_PARAMS bls_verify prove --wasm ./pkg/output.wasm --output ./output --private $1:i64)
# $ZKWASM_CLI --params $ZKWASM_PARAMS bls_verify verify --wasm ./pkg/output.wasm --output ./output
