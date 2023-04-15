// Welcome to the interactive Rust file for Section 2!
// You can use this file to practice Rust concepts as you learn them.
//
// To run this file, make sure you have Rust and Cargo installed.
// Navigate to the directory containing this file in your terminal,
// and run the command 'cargo run --bin solution2' (without quotes) to execute the code.

fn main() {
    // 1. Use an if-else expression to print whether a variable `x` is even or odd.
    // Declare a variable `x` and initialize it with a number.
    let x = 7;
    if x % 2 == 0 {
        println!("x is even");
    } else {
        println!("x is odd");
    }

    // 2. Use a for loop to print the numbers 1 through 5.
    for i in 1..=5 {
        println!("i: {}", i);
    }

    // 3. Use a while loop to print the numbers 5 through 1 in descending order.
    let mut count = 5;
    while count > 0 {
        println!("count: {}", count);
        count -= 1;
    }

    // 4. Use a loop construct to print the numbers 10 through 20, then break the loop.
    let mut counter = 10;
    loop {
        println!("counter: {}", counter);
        counter += 1;
        if counter > 20 {
            break;
        }
    }
}
