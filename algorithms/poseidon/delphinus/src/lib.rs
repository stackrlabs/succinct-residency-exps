use wasm_bindgen::prelude::*;
use zkwasm_rust_sdk::{
    wasm_input,
    dbg,
};
use wasm::poseidon_hash;

#[wasm_bindgen]
pub fn zkmain() {
    let iterations = unsafe { wasm_input(0) };
    poseidon_hash(iterations.try_into().unwrap());
    dbg!("done\n");
}
