#![cfg_attr(target_arch = "riscv32", no_std, no_main)]
extern crate alloc;
use alloc::vec::Vec;
use ark_std::println;
use ark_std::str::FromStr;
use ark_bn254::Fr;
use poseidon_ark::Poseidon;
use nexus_rt::read_private_input;

// Reimplemented here and not using Wasm package because of no_std
fn poseidon_hash(arr_len: u32) -> u32 {
    let mut input_arr: Vec<Fr> = Vec::with_capacity(arr_len as usize);
    for i in 0..arr_len as usize {
        input_arr.push(Fr::from_str(&i.to_string()).unwrap());
    }
    let poseidon = Poseidon::new();
    let hash = poseidon.hash(input_arr.clone()).unwrap();
    println!("Array Length: {:?}", arr_len);
    println!("Hash: {:?}", hash);
    1
}

#[nexus_rt::profile]
fn poseidon_hash_exec(n: u32) -> u32 {
    poseidon_hash(n)
}


#[nexus_rt::main]
fn main() {
    let n = read_private_input::<u32>().expect("failed to read input");
    let result = poseidon_hash_exec(n);
    assert_eq!(1, result);
}
