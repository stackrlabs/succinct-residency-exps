use wasm_bindgen::prelude::*;
use zkwasm_rust_sdk::{
    wasm_input,
    dbg,
};
use wasm::nth_prime;

#[wasm_bindgen]
pub fn zkmain() {
    let n = unsafe { wasm_input(0) };
    nth_prime(n.try_into().unwrap());
    dbg!("done\n");
}
