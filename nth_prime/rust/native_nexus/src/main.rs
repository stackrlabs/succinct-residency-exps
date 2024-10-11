use nexus_sdk::{
    compile::CompileOpts,
    nova::seq::{Generate, Nova, PP},
    Local, Prover, Verifiable,
};
 
const PACKAGE: &str = "guest";
 
fn generate_pp_and_compile() -> (PP, Nova<Local>) {
    let pp: PP = PP::generate().expect("failed to generate parameters");
    let opts = CompileOpts::new(PACKAGE);
 
    let prover: Nova<Local> = Nova::compile(&opts).expect("failed to compile guest program");
 
    (pp, prover)
}
 
fn prove_execution(pp: &PP, prover: Nova<Local>) -> nexus_sdk::nova::seq::Proof {
    prover.prove(pp).expect("failed to prove program")
}
 
fn verify_execution(pp: &PP, proof: &nexus_sdk::nova::seq::Proof) {
    proof.verify(pp).expect("failed to verify proof");
}
 
fn main() {
    use std::time::Instant;
 
    let start = Instant::now();
    let (pp, prover) = generate_pp_and_compile();
    let duration = start.elapsed();
    println!(
        "Time taken to generate PP and compile: {:.2} seconds",
        duration.as_secs_f64()
    );
 
    let start = Instant::now();
    let proof = prove_execution(&pp, prover);
    let duration = start.elapsed();
    println!(
        "Time taken to prove execution: {:.2} seconds",
        duration.as_secs_f64()
    );
 
    let start = Instant::now();
    verify_execution(&pp, &proof);
    let duration = start.elapsed();
    println!(
        "Time taken to verify execution: {:.2} seconds",
        duration.as_secs_f64()
    );
}
