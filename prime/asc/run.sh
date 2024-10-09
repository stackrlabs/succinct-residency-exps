#!/bin/bash
: '
Make sure you have zkWasm installed and compiled.
Use this if not:
    git clone git@github.com:DelphinusLab/zkWasm.git
    cd zkWasm
    git submodule update --init
    cargo build --release
'

asc is_prime.ts -O --noAssert -o prime-zk.wasm

cd ../../zkWasm || exit

RUST_LOG=info cargo run --release -- --params ./params testwasm setup --host standard -k 18 --wasm ../prime/asc/prime-zk.wasm
RUST_LOG=info cargo run --release -- --params ./params testwasm prove --output ./output --ctxout ctxout --wasm ../prime/asc/prime-zk.wasm --public 9999991:i64 --private 1:i64
RUST_LOG=info cargo run --release -- --params ./params testwasm verify --output ./output
