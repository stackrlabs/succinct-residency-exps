// These constants represent the RISC-V ELF and the image ID generated by risc0-build.
// The ELF is used for proving and the ID is used for verification.
use methods::{GUEST_CODE_FOR_BLOCK_VERIFICATION_ELF, GUEST_CODE_FOR_BLOCK_VERIFICATION_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

fn main() {
    // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    let wasm =
        include_bytes!("../../../wasm/target/wasm32-unknown-unknown/release/wasm.wasm").to_vec();

    let file = File::open("../../../inputs/block.json").expect("Failed to open file");
    let mut block = Vec::new();
    let mut reader = BufReader::new(file);
    reader.read_to_end(&mut block).expect("Failed to read file");

    let env = ExecutorEnv::builder()
        .write(&wasm)
        .unwrap()
        .write(&block)
        .unwrap()
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Proof information by proving the specified ELF binary.
    // This struct contains the receipt along with statistics about execution of the guest
    let prove_info = prover.prove(env, GUEST_CODE_FOR_BLOCK_VERIFICATION_ELF).unwrap();

    let receipt = prove_info.receipt;
    let _output: u32 = receipt.journal.decode().unwrap();

    // The receipt was verified at the end of proving, but the below code is an
    // example of how someone else could verify this receipt.
    receipt.verify(GUEST_CODE_FOR_BLOCK_VERIFICATION_ID).unwrap();
}