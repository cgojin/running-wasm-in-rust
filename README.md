# running-wasm-in-rust

Running wasm in Rust.

## Converting WebAssembly text format to wasm

```sh
# install wasm-tools, frist only
cargo install wasm-tools

# hello.wat to hello.wasm
wasm-tools parse hello.wat -o hello.wasm
```

## Running wasm in Rust

```sh
cargo run
```

## References

- [wasmtime](https://github.com/bytecodealliance/wasmtime)
