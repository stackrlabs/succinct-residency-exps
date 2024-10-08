// use wasmer::{Store, Module, Instance, Value, imports};

// fn main() -> anyhow::Result<()> {
//     let module_wat = std::fs::read_to_string("nth_fib.wat")?;

//     let mut store = Store::default();
//     let module = Module::new(&store, &module_wat)?;
//     // The module doesn't import anything, so we create an empty import object.
//     let import_object = imports! {};
//     let instance = Instance::new(&mut store, &module, &import_object)?;

//     let fib = instance.exports.get_function("fib")?;
//     let result = fib.call(&mut store, &[Value::I32(42)])?;
//     assert_eq!(result[0], Value::I32(43));

//     Ok(())
// }
