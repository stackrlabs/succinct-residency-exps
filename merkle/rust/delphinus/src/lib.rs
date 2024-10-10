use wasm_bindgen::prelude::*;
use zkwasm_rust_sdk::{
    wasm_input,
    dbg,
};
use wasm::merkelize_impl;

#[wasm_bindgen]
pub fn zkmain() {
    let leaves = (0..10)
        .map(|i| i.to_string().as_bytes().to_vec())
        .collect::<Vec<_>>();
    // will panic if merklize uses log or sqrt
    let res = merkelize_impl(leaves);
    dbg!("done! {:?}\n", res);
}
