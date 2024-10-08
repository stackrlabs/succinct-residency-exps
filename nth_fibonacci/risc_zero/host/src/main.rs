use methods::{GUEST_NTH_FIB_ELF, GUEST_NTH_FIB_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    let wasm = include_bytes!("../fib.wasm").to_vec();
    let n: i32 = 20;
    let env = ExecutorEnv::builder()
        .write(&wasm)
        .unwrap()
        .write(&n)
        .unwrap()
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Proof information by proving the specified ELF binary.
    // This struct contains the receipt along with statistics about execution of the guest
    let prove_info = prover.prove(env, GUEST_NTH_FIB_ELF).unwrap();

    // extract the receipt.
    let receipt = prove_info.receipt;

    let _output: u32 = receipt.journal.decode().unwrap();

    receipt.verify(GUEST_NTH_FIB_ID).unwrap();
}
