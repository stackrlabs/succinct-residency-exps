#!/bin/bash

set -e
set -x

rm -rf output
mkdir output

ZKWASM_DIR="../../../zkWasm" # needs pre-installed
ZKWASM_CLI=$ZKWASM_DIR/target/release/zkwasm-cli
ZKWASM_PARAMS=$ZKWASM_DIR/params

$ZKWASM_CLI --params $ZKWASM_PARAMS merkle-proof setup
$ZKWASM_CLI --params $ZKWASM_PARAMS merkle-proof dry-run --wasm ./pkg/output.wasm --output ./output --private $1:i64
time ($ZKWASM_CLI --params $ZKWASM_PARAMS merkle-proof prove --wasm ./pkg/output.wasm --output ./output --private $1:i64)
# $ZKWASM_CLI --params $ZKWASM_PARAMS merkle-proof verify --wasm ./pkg/output.wasm --output ./output