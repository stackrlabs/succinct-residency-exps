# succinct-residency-exps

## Run Benchmarks

```bash
make all
```

To run individual experiments, use the following commands:

```bash
make <rust-native rust-sp1 rust-wasm-sp1>
```

## Results (TO BE UPDATED)

| Language | Runtime | Prover | Binary Search | Is Prime    | Merklization | Block Header Verification | TSP         |
| --------- | ------- | ------ | ------------- | ----------- | ------------ | ------------------------- | ----------- |
| Rust      | Native  | -      | 0.000000000   | 0.000000000 | 0.000000000  | 0.000000000               | 0.000000000 |
| Rust      | Native  | SP1    | 0.000000000   | 0.000000000 | 0.000000000  | 0.000000000               | 0.000000000 |
| Rust      | WASM    | SP1    | 0.000000000   | 0.000000000 | 0.000000000  | 0.000000000               | 0.000000000 |
| JS        | WASM (QuickJS)    | -    | 0.000000000   | 0.000000000 | 0.000000000  | 0.000000000               | 0.000000000 |
| JS        | WASM (SpiderMonkey)    | -    | 0.000000000   | 0.000000000 | 0.000000000  | 0.000000000               | 0.000000000 |
| JS        | WASM (Boa + Rust)   | SP1    | 0.000000000   | 0.000000000 | 0.000000000  | 0.000000000               | 0.000000000 |
| Go        | WASM (WASI) | - | | 0.000000000   | 0.000000000 | 0.000000000  | 0.000000000               | 0.000000000 |
| Go        | WASM (WASI) | SP1 | | 0.000000000   | 0.000000000 | 0.000000000  | 0.000000000               | 0.000000000 |
