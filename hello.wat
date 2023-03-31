(module
  (func $hello (import "imports" "hello") (param i32))
  (func $add (import "imports" "add") (param i32 i32) (result i32))
  ;; (import "imports" "rust_add" (func $rust_add (param i32 i32) (result i32)))

  (func (export "wasm_hello")
    i32.const 2
    i32.const 3
    call $add
    call $hello
  )

  (func (export "wasm_add") (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.add
  )
)