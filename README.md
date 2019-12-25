# wasm_cream

a throwaway project for running rust on wasm (wasi) on rust; wasm is the cream in the middle

## useful docs

- [wasmtime docs](https://bytecodealliance.github.io/wasmtime/embed-rust.html)
- [Rust WASI example](https://github.com/bytecodealliance/wasmtime/blob/master/docs/WASI-tutorial.md#from-rust)
- https://github.com/bytecodealliance/wasmtime/blob/master/docs/wasm-rust.md

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

### Basic

From top-level:

     ./rust_wasi_runtime/target/debug/rust_wasi_runtime --file ./rust_wasi_lib/target/wasm32-wasi/debug/rust_wasi_lib.wasm

This runs the default `entry_point`, which doesn't do anything special.

### Threaded

There is also the following, which attempts to use threads:

    ./rust_wasi_runtime/target/debug/rust_wasi_runtime --file ./rust_wasi_lib/target/wasm32-wasi/debug/rust_wasi_lib.wasm --function threaded_entry_point

However, as of end of 2019, this returns the following:

    Loading module at ./rust_wasi_lib/target/wasm32-wasi/debug/rust_wasi_lib.wasm
    Creating WASI module instance
    Resolving imports of ./rust_wasi_lib/target/wasm32-wasi/debug/rust_wasi_lib.wasm
    Module: Name("wasi_unstable"), Field: Name("fd_write")
    Creating instance for ./rust_wasi_lib/target/wasm32-wasi/debug/rust_wasi_lib.wasm module
    Finding threaded_entry_point
    Threaded
    thread '<unnamed>' panicked at 'failed to spawn thread: Custom { kind: Other, error: "operation not supported on wasm yet" }', src/libcore/result.rs:1165:5
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
    thread 'main' panicked at 'success: Ref(Trap { message: "wasm trap: unreachable, source location: @da0a" })', src/libcore/result.rs:1165:5
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

What I think is happening here is that rust-on-wasi is panicking because threads aren't supported
yet by `wasm32-wasi` target, whilst wasi-on-rust is panicking because the wasm call failed.
