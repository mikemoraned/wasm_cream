extern crate clap;
use clap::{App, Arg};

// use core::borrow::Borrow;
use std::collections::HashMap;
use std::fs::read;
use wasmtime::*;
use wasmtime_jit::*;
// use wasmtime_wasi::old::snapshot_0::create_wasi_instance as create_wasi_instance_snapshot_0;

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

    // let mut features = Features::default();
    // features.threads = true;
    let mut config = Config::new().wasm_threads(true);
    // let config = config.features(features);
    // let engine = HostRef::new(Engine::new(&config));
    let engine = Engine::new(&config);
    // let store = HostRef::new(Store::new(&engine));
    let store = Store::new(&engine);

    let wasm_path = matches.value_of("file").unwrap();
    let wasm = read(wasm_path).expect("wasm file");

    println!("Loading module at {}", wasm_path);

    // let module = HostRef::new(Module::new(&store, &wasm).expect("wasm module"));
    let module = Module::new(&store, &wasm).expect("wasm module");

    println!("Creating WASI module instance");
    let preopened_dirs = vec![];
    let argv = vec![];
    let environ = vec![];
    let wasi_unstable =
        create_wasi_instance_snapshot_0(&store, &preopened_dirs, &argv, &environ).unwrap();
    let mut module_registry = HashMap::new();
    module_registry.insert("wasi_unstable".to_owned(), wasi_unstable);

    println!("Resolving imports of {}", wasm_path);
    let mut imports = vec![];
    // for i in module.borrow().imports() {
    for i in module.imports() {
        let module_name = i.module();
        let field_name = i.name();
        println!("Module: {:?}, Field: {:?}", module_name, field_name);
        if let Some(instance) = module_registry.get(module_name) {
            if let Some(export) = instance.borrow().find_export_by_name(field_name) {
                imports.push(export.clone());
            }
        }
    }

    println!("Creating instance for {} module", wasm_path);
    // let instance = Instance::new(&store, &module, &imports).expect("wasm instance");
    let instance = Instance::new(&module, &imports).expect("wasm instance");

    let function_name = matches.value_of("function").unwrap();

    println!("Finding {}", function_name);

    let answer = instance
        .find_export_by_name(function_name)
        .expect("find exported name")
        .func()
        .expect("function");
    let result = answer.borrow().call(&[]).expect("success");
    println!("Result: {}", result[0].i32());
}
