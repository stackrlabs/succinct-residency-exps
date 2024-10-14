#![cfg_attr(target_arch = "riscv32", no_std, no_main)]
extern crate alloc;
use alloc::vec;

#[nexus_rt::profile]
fn nth_prime_exec(n: u32) -> u64 {
    nth_prime(n)
}

// natively implemented so that can use alloc::vec
// std::vec is not available in the scope
fn nth_prime(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut primes = vec![2];
    let mut candidate = 3;
    while primes.len() < n as usize {
        if primes.iter().all(|&p| candidate % p != 0) {
            primes.push(candidate);
        }
        candidate += 2;
    }
    primes[n as usize - 1]
}
 
#[nexus_rt::main]
fn main() {
    let n = 1;
    let result = nth_prime_exec(n);
    assert_eq!(2, result);
}
