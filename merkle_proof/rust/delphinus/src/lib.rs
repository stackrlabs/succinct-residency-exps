use wasm_bindgen::prelude::*;
use zkwasm_rust_sdk::{
    wasm_input,
    dbg,
};
use wasm::generate_merkle_proof;

#[wasm_bindgen]
pub fn zkmain() {
    let num_leaves = unsafe { wasm_input(0) };
    generate_merkle_proof(num_leaves.try_into().unwrap());
    dbg!("done\n");
}
