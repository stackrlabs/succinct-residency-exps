use wasm_bindgen::prelude::*;
use zkwasm_rust_sdk::{
    wasm_input,
    dbg,
};
use wasm::bls_aggregate;

#[wasm_bindgen]
pub fn zkmain() {
    let num_signatures = unsafe { wasm_input(0) };
    bls_aggregate(num_signatures.try_into().unwrap());
    dbg!("done\n");
}
