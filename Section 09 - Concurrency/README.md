# Section 9: Concurrency

In this section, we'll cover concurrency in Rust, comparing it to the concurrency concepts in TypeScript.

Concurrency is the execution of multiple tasks simultaneously, allowing your program to perform multiple operations at the same time. Both Rust and TypeScript support concurrency, but they use different approaches.

## Rust Concurrency with Threads

Rust uses the concept of threads to achieve concurrency. A thread is the smallest unit of execution within a process, and multiple threads can run concurrently within a single process.

Rust provides the `std::thread` module to handle threads. It allows you to create new threads, join threads, and share data between threads using channels or synchronization primitives like Mutex, RwLock, etc.

### Creating and Joining Threads

To create a new thread, you can use the `spawn` function from the `std::thread` module:

#### Rust

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Thread: {}", i);
        }
    });

    for i in 1..5 {
        println!("Main: {}", i);
    }

    handle.join().unwrap(); // wait for the spawned thread to finish
}
```

### Sharing Data Between Threads

To share data between threads in Rust, you can use `Arc` (atomic reference counting) for shared ownership and `Mutex` for exclusive access to the shared data:

#### Rust

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
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
```

## TypeScript Concurrency with Promises and Async/Await

In TypeScript, concurrency is achieved using asynchronous programming with Promises and the async/await syntax. Promises represent the result of an asynchronous operation that might not have completed yet. The async/await syntax makes working with Promises more readable and easier to manage.

### Promises

Promises can be in one of three states: pending, fulfilled, or rejected. You can attach callbacks to handle the fulfilled or rejected state using the then and catch methods, respectively:

#### Typescript

```typescript
const promise = new Promise<string>((resolve, reject) => {
    setTimeout(() => {
        resolve("Promise fulfilled!");
    }, 1000);
});

promise
    .then((value) => {
        console.log(value);
    })
    .catch((error) => {
        console.error(error);
    });
```

### Async/Await

The async/await syntax allows you to write asynchronous code that looks more like synchronous code:

#### Typescript

```typescript
async function fetchData(): Promise<string> {
    return new Promise((resolve) => {
        setTimeout(() => {
            resolve("Data fetched!");
        }, 1000);
    });
}

async function main() {
    try {
        const data = await fetchData();
        console.log(data);
    } catch (error) {
        console.error(error);
    }
}

main();
```

Now that you have a basic understanding of Rust's concurrency and how it compares to TypeScript, practice them in the `section9.rs`.
