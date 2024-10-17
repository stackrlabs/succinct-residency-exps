use nexus_sdk::{
    compile::CompileOpts,
    nova::seq::{Generate, Nova, PP},
    Local, Prover, Verifiable,
};
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;

const PACKAGE: &str = "guest";
type Input = u32;
 
fn generate_pp_and_compile() -> (PP, Nova<Local>) {
    let pp: PP = PP::generate().expect("failed to generate parameters");
    let opts = CompileOpts::new(PACKAGE);
 
    let prover: Nova<Local> = Nova::compile(&opts).expect("failed to compile guest program");
 
    (pp, prover)
}
 
fn prove_execution(pp: &PP, prover: Nova<Local>) -> nexus_sdk::nova::seq::Proof {
    let file = File::open("../../../inputs/merkle.json").expect("Failed to open input file");
    let reader = BufReader::new(file);
    let json: Value = serde_json::from_reader(reader).expect("Failed to parse JSON");
    // Extract the number from the JSON
    let input = json["numLeaves"].as_u64().expect("Failed to parse numLeaves from JSON") as Input;
    println!("Input numLeaves read from JSON: {}", input);
    prover.prove_with_input::<Input>(pp, &input).expect("failed to prove program")
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
