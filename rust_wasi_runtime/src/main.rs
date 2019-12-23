extern crate clap;
use clap::{App, Arg};

use std::fs::read;
use wasmtime::*;

fn main() {
    let matches = App::new("Rust WASI Runtime")
        .about("Runs WASM programs that use WASI")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("path to wasm file to run")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("function")
                .long("function")
                .value_name("FUNCTION_NAME")
                .help("name of function to run")
                .default_value("entry_point")
                .takes_value(true),
        )
        .get_matches();

    let engine = HostRef::new(Engine::default());
    let store = HostRef::new(Store::new(&engine));

    let wasm_path = matches.value_of("file").unwrap();
    let wasm = read(wasm_path).expect("wasm file");

    let module = HostRef::new(Module::new(&store, &wasm).expect("wasm module"));
    let instance = Instance::new(&store, &module, &[]).expect("wasm instance");

    let function_name = matches.value_of("function").unwrap();

    let answer = instance
        .find_export_by_name(function_name)
        .expect("find exported name")
        .func()
        .expect("function");
    let result = answer.borrow().call(&[]).expect("success");
    println!("Result: {}", result[0].i32());
}
