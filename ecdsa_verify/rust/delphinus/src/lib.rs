use wasm_bindgen::prelude::*;
use zkwasm_rust_sdk::{
    wasm_input,
    dbg,
};
use wasm::ecdsa_verify_n;

#[wasm_bindgen]
pub fn zkmain() {
    let iterations = unsafe { wasm_input(0) };
    ecdsa_verify_n(iterations.try_into().unwrap());
    dbg!("done\n");
}
