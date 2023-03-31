use anyhow::Result;
use wasmtime::{Caller, Engine, Func, Instance, Module, Store};

struct State {
    sum: i32,
}

fn main() -> Result<()> {
    let engine = Engine::default();

    // let module = Module::from_file(&engine, "hello.wasm")?;
    let module = Module::from_file(&engine, "hello.wat")?;

    let mut store = Store::new(
        &engine,
        State {
            sum: 0,
        },
    );

    let add = Func::wrap(&mut store, |a: i32, b: i32| {
        println!("> wasm call rust: add({}, {})", a, b);
        a + b
    });
    let hello = Func::wrap(&mut store, |mut caller: Caller<'_, State>, sum: i32| {
        println!("> wasm call rust: hello({})", sum);
        caller.data_mut().sum = sum;
    });

    let imports = [hello.into(), add.into()];
    let instance = Instance::new(&mut store, &module, &imports)?;

    let wasm_hello = instance.get_typed_func::<(), ()>(&mut store, "wasm_hello")?;
    let wasm_add = instance.get_typed_func::<(i32, i32), i32>(&mut store, "wasm_add")?;

    wasm_hello.call(&mut store, ())?;
    let sum = wasm_add.call(&mut store, (2, 3))?;
    println!("> rust call wasm: wasm_add(2, 3) = {}", sum);

    println!("store.data().sum: {}", store.data().sum);
    Ok(())
}
