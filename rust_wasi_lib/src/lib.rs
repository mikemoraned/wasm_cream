use std::thread;

#[no_mangle]
pub extern "C" fn entry_point() -> i32 {
    println!("Simple");
    42
}

#[no_mangle]
pub extern "C" fn threaded_entry_point() -> i32 {
    println!("Threaded");
    let spawned = thread::spawn(|| 42);
    println!("Joining");
    spawned.join().unwrap()
}
