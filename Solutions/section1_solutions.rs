// Welcome to the interactive Rust file for Section 1!
// You can use this file to practice Rust concepts as you learn them.
//
// To run this file, make sure you have Rust and Cargo installed.
// Navigate to the directory containing this file in your terminal,
// and run the command 'cargo run --bin solution1' (without quotes) to execute the code.

fn main() {
    // 1. Declare an immutable variable `x` with the value 5
    let x = 5;
    println!("x: {}", x);

    // 2. Declare a mutable variable `y` with the value 6
    let mut y = 6;
    println!("y: {}", y);

    // 3. Modify the value of `y` to 7
    y = 7;
    println!("y: {}", y);

    // 4. Declare an f64 variable `z` with the value 3.14
    let z: f64 = 3.14;
    println!("z: {}", z);

    // 5. Declare a boolean variable `is_active` with the value true
    let is_active = true;
    println!("is_active: {}", is_active);

    // 6. Declare a char variable `letter` with the value 'A'
    let letter = 'A';
    println!("letter: {}", letter);

    // 7. Declare a string slice variable `greeting` with the value "Hello, Rust!"
    let greeting: &str = "Hello, Rust!";
    println!("greeting: {}", greeting);

    // 8. Declare a mutable String variable `message` with the value "Welcome to Rust!"
    let mut message: String = String::from("Welcome to Rust!");
    println!("message: {}", message);

    // 9. Modify the value of `message` by appending " Enjoy your learning journey!"
    message.push_str(" Enjoy your learning journey!");
    println!("message: {}", message);
}
