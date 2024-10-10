use wasm_bindgen::prelude::*;
use zkwasm_rust_sdk::{
    wasm_input,
    dbg,
};
use wasm::merkelize;

#[wasm_bindgen]
pub fn zkmain() {
    let num_leaves = unsafe { wasm_input(0) };
    merkelize(num_leaves.try_into().unwrap());
    dbg!("done\n");
}
