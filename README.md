# succinct-residency-exps

This repo contains benchmarks conducted by the Stackr team at the Succinct ZK Residency [program.](https://blog.succinct.xyz/zk-residency/)

The following zkVMs were tested:

- SP1
- RISC0
- Jolt
- Nexus
- Delphinus

with 8 algorithms of varying complexities: `nth_prime`, `ECDSA_verify`, `BLS verification`, `BLS signature aggregation`, `Keccak hashing`, `Poseidon hashing`, `Merkle tree generation` and `Merkle proof inclusion`.

Benchmark numbers are [here.](https://docs.google.com/spreadsheets/d/1HwZQkgiUro9Nl30tO3KdXizB_D-1_J3vIVXbTvuY2Mw/edit?usp=sharing)

Slides and recorded talk will be available soon.

## Run Benchmarks

### Prerequisites

- `rust`
- `wasm-pack`
- zkVM-specific dependencies

### Run
To run all benchmarks, use the following command:

```bash
make all
```

To run individual benchmarks, use the following commands:

```bash
make run-<algorithm>
```
