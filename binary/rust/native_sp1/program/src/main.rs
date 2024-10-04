//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use binary::binary_search;

pub fn main() {
    println!("cycle-tracker-start: input");
    let search_list = sp1_zkvm::io::read::<Vec<i32>>();
    let search_value = sp1_zkvm::io::read::<i32>();
    println!("cycle-tracker-end: input");
    println!("cycle-tracker-start: execution");
    let res = binary_search(search_list, search_value);
    println!("cycle-tracker-end: execution");
    println!("binary_search: found number: {}", res);

    sp1_zkvm::io::commit(&res);
}
