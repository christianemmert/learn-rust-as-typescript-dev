// Welcome to the interactive Rust file for Section 9!
// You can use this file to practice Rust concepts as you learn them.
//
// To run this file, make sure you have Rust and Cargo installed.
// Navigate to the directory containing this file in your terminal,
// and run the command 'cargo run --bin solution9' (without quotes) to execute the code.

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // 1. Create a new thread that prints "Hello from thread!" and wait for it to finish
    let handle = thread::spawn(|| {
        println!("Hello from thread!");
    });
    handle.join().unwrap();

    // 2. Create two threads, one that prints "Thread 1" and another that prints "Thread 2"
    // Wait for both threads to finish
    let handle1 = thread::spawn(|| {
        println!("Thread 1");
    });
    let handle2 = thread::spawn(|| {
        println!("Thread 2");
    });
    handle1.join().unwrap();
    handle2.join().unwrap();

    // 3. Create a shared counter using Arc and Mutex, increment it by 1 in 5 different threads
    // Wait for all threads to finish and print the final value of the counter
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
