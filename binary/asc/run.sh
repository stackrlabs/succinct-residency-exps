#!/bin/bash
: '
Make sure you have zkWasm installed and compiled.
Use this if not:
    git clone git@github.com:DelphinusLab/zkWasm.git
    cd zkWasm
    git submodule update --init
    cargo build --release
'

# when using array you have to disable bulk-memory and use custom abort function
asc bin_search.ts -O --noAssert -o binary-zk.wasm --disable bulk-memory --runtime stub --use abort=bin_search/abort

cd ../../zkWasm || exit

RUST_LOG=info cargo run --release -- --params ./params testwasm setup --host standard -k 18 --wasm ../binary/asc/binary-zk.wasm
RUST_LOG=info cargo run --release -- --params ./params testwasm prove --output ./output --ctxout ctxout --wasm ../binary/asc/binary-zk.wasm --public 19:i64 &>../binary/asc/prove.log
RUST_LOG=info cargo run --release -- --params ./params testwasm verify --output ./output
