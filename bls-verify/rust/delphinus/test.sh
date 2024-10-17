#!/bin/bash

set -e
set -x

rm -rf output
mkdir output

ZKWASM_DIR="../../../../zkWasm" # needs pre-installed
ZKWASM_CLI=$ZKWASM_DIR/target/release/zkwasm-cli
ZKWASM_PARAMS=$ZKWASM_DIR/params

$ZKWASM_CLI --params $ZKWASM_PARAMS bls_verify setup --wasm ./pkg/output.wasm
$ZKWASM_CLI --params $ZKWASM_PARAMS bls_verify dry-run --wasm ./pkg/output.wasm --output ./output --private $1:f64
# time ($ZKWASM_CLI --params $ZKWASM_PARAMS bls_aggregate prove --wasm ./pkg/output.wasm --output ./output --private $1:f64)
# $ZKWASM_CLI --params $ZKWASM_PARAMS bls_aggregate verify --wasm ./pkg/output.wasm --output ./output
