// Copyright 2024 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use risc0_zkvm::guest::env;
use wasmi::{Engine, Linker, Module, Store};

fn main() {
    let start = env::cycle_count();
    let engine = Engine::default();

    let wasm: Vec<u8> = env::read();
    let n: i32 = env::read();

    // Derived from the wasmi example: https://docs.rs/wasmi/0.29.0/wasmi/#example
    let module = Module::new(&engine, &mut &wasm[..]).expect("Failed to create module");
    type HostState = u32;

    let linker = <Linker<HostState>>::new(&engine);
    let mut store = Store::new(&engine, 42);
    let instance = linker
        .instantiate(&mut store, &module)
        .expect("failed to instantiate")
        .start(&mut store)
        .expect("Failed to start");

    let fib = instance
        .get_typed_func::<i32, i32>(&store, "fib")
        .expect("Failed to get typed_func");

    let fib_call_start = env::cycle_count();
    let res = fib.call(&mut store, n).expect("Failed to call");
    let fib_call_end = env::cycle_count();
    eprintln!("fib_call_cycles: {}", fib_call_end - fib_call_start);

    env::log(&format!("fib {} - {}", n, res));
    env::commit(&res);
    let end = env::cycle_count();
    eprintln!("total_cycles: {}", end - start);
}
