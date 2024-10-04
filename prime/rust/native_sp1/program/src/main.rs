//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use prime::is_prime;

pub fn main() {
    let input = sp1_zkvm::io::read::<i32>();
    println!("cycle-tracker-start: execution");
    let res = is_prime(input);
    println!("cycle-tracker-end: execution");
    println!("is_prime {} - {}", input, res);

    sp1_zkvm::io::commit(&res);
}
