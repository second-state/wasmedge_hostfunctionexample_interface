# WasmEdge Host Function Example Interface

A Rust library that provides Rust to WebAssembly developers with syntax for running self-defined host functions when their Wasm is being executed on [WasmEdge](https://github.com/WasmEdge/WasmEdge) (formerly `SSVM`).

# How to use this library

## Rust dependency

Developers will add the [`wasmedge_hostfunctionexample_interface` crate](https://crates.io/crates/wasmedge_hostfunctionexample_interface) as a dependency to their `Rust -> Wasm` applications. For example, add the following line to the application's `Cargo.toml` file.
```
[dependencies]
wasmedge_hostfunctionexample_interface = "^1.0.0"
```

