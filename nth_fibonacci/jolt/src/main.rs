pub fn main() {
    let wasm = include_bytes!("../../fib.wasm").to_vec();
    let (prove_fib, verify_fib) = guest::build_fib();

    let (output, proof) = prove_fib(20);
    let is_valid = verify_fib(proof);

    println!("output: {}", output);
    println!("valid: {}", is_valid);
}
