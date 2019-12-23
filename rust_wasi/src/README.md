# About

Example based on [Ruse WASI example](https://github.com/bytecodealliance/wasmtime/blob/master/docs/WASI-tutorial.md#from-rust)

## Install tools

    rustup target add wasm32-wasi

    curl https://wasmtime.dev/install.sh -sSf | bash

## Compile and run

    cargo build --target wasm32-wasi
    wasmtime target/wasm32-wasi/debug/rust_wasi.wasm
