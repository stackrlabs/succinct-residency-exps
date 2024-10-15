#![no_main]

use wasmi::{Engine, Linker, Module, Store};

const PAGE_SIZE: u32 = 65536;

#[jolt::provable(stack_size = 1000000, memory_size = 10000000, max_input_size = 10000000)]
pub fn bls_verify(num_signers: u32, aggregate_signature: &[u8], wasm: &[u8]) -> u32 {
    let engine = Engine::default();
    let module = Module::new(&engine, &mut &wasm[..]).expect("Failed to create module");

    let mut linker = <Linker<()>>::new(&engine);
    let mut store = Store::new(&engine, ());

    let instance = linker
        .instantiate(&mut store, &module)
        .unwrap()
        .start(&mut store)
        .unwrap();

    // write list to memory
    let memory = instance.get_memory(&store,"memory").expect("Failed to get memory");
    let ptr = memory.data_size(&mut store) as i32;

    // grow memory to fit the block
    let sig_len = aggregate_signature.len();
    let memory_size = (sig_len as u32 + PAGE_SIZE - 1) / PAGE_SIZE; // round up to the nearest 64KiB page range
    memory.grow(&mut store, memory_size).expect("Failed to grow memory");
    memory.write(&mut store, ptr as usize, &aggregate_signature).expect("Failed to write to memory");

    let bls_verify = instance
        .get_typed_func::<(u32, i32, i32), u32>(&mut store, "bls_verify_wasm")
        .expect("Failed to get typed_func");
    let res = bls_verify.call(&mut store, (num_signers, ptr, sig_len as i32)).expect("Failed to call");
    println!("bls_verify - {}", res);
    res
}
