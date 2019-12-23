# wasm_cream

a throwaway project for running rust on wasm (wasi) on rust; wasm is the cream in the middle

## useful docs

- [wasmtime docs](https://bytecodealliance.github.io/wasmtime/embed-rust.html)
- [Rust WASI example](https://github.com/bytecodealliance/wasmtime/blob/master/docs/WASI-tutorial.md#from-rust)

## Install tools

Assuming Rust toolchain already installed, you still need:

    rustup target add wasm32-wasi

## Compile

### Rust library -> WASM

In `rust_wasi_lib`:

    cargo build --target wasm32-wasi

### Rust WASI Runtime

In `rust_wasi_runtime`:

    cargo build

## WASM lib on Runtime

From top-level:

     ./rust_wasi_runtime/target/debug/rust_wasi_runtime --file ./rust_wasi_lib/target/wasm32-wasi/debug/rust_wasi_lib.wasm
