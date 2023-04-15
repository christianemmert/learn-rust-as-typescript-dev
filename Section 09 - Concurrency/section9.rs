// Welcome to the interactive Rust file for Section 9!
// You can use this file to practice Rust concurrency concepts as you learn them.
//
// To run this file, make sure you have Rust and Cargo installed.
// Navigate to the directory containing this file in your terminal,
// and run the command 'cargo run --bin section9' (without quotes) to execute the code.

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // 1. Create a new thread that prints "Hello from thread!" and wait for it to finish
    // Your code here

    // 2. Create two threads, one that prints "Thread 1" and another that prints "Thread 2"
    // Wait for both threads to finish
    // Your code here

    // 3. Create a shared counter using Arc and Mutex, increment it by 1 in 5 different threads
    // Wait for all threads to finish and print the final value of the counter
    // Your code here
}
