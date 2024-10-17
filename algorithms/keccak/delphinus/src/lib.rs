use wasm_bindgen::prelude::*;
use zkwasm_rust_sdk::{
    wasm_input,
    dbg,
};
use wasm::keccak_n;

#[wasm_bindgen]
pub fn zkmain() {
    let iterations = unsafe { wasm_input(0) };
    keccak_n(iterations.try_into().unwrap());
    dbg!("done\n");
}
