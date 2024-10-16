use wasm_bindgen::prelude::*;
use zkwasm_rust_sdk::{
    wasm_input,
    dbg,
};
use wasm::bls_verify;

#[wasm_bindgen]
pub fn zkmain() {
    let num_signatures = unsafe { wasm_input(0) };
    // TODO: make first argument a list of public keys
    bls_verify(num_signatures.try_into().unwrap());
    dbg!("done\n");
}
