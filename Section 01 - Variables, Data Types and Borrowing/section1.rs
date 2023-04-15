// Welcome to the interactive Rust file for Section 1!
// You can use this file to practice Rust concepts as you learn them.
//
// To run this file, make sure you have Rust and Cargo installed.
// Navigate to the directory containing this file in your terminal,
// and run the command 'cargo run --bin section1' (without quotes) to execute the code.

fn main() {
    // 1. Declare an immutable variable `x` with the value 5
    // Your code here
    println!("x: {}", x);

    // 2. Declare a mutable variable `y` with the value 6
    // Your code here
    println!("y: {}", y);

    // 3. Modify the value of `y` to 7
    // Your code here
    println!("y: {}", y);

    // 4. Declare an f64 variable `z` with the value 3.14
    // Your code here
    println!("z: {}", z);

    // 5. Declare a boolean variable `is_active` with the value true
    // Your code here
    println!("is_active: {}", is_active);

    // 6. Declare a char variable `letter` with the value 'A'
    // Your code here
    println!("letter: {}", letter);

    // 7. Declare a string slice variable `greeting` with the value "Hello, Rust!"
    // Your code here
    println!("greeting: {}", greeting);

    // 8. Declare a mutable String variable `message` with the value "Welcome to Rust!"
    // Your code here
    println!("message: {}", message);

    // 9. Modify the value of `message` by appending " Enjoy your learning journey!"
    // Your code here
    println!("message: {}", message);

    // 10. Declare a tuple variable `tuple_example` with the values (42, 3.14, true)
    // Your code here
    println!("tuple_example: {:?}", tuple_example);

    // 11. Declare an array variable `array_example` with the values [1, 2, 3, 4, 5]
    // Your code here
    println!("array_example: {:?}", array_example);

    // 12. Write a function `calculate_length` that takes a string slice reference and returns its length
    // Your code here
    let len = calculate_length(&greeting);
    println!("The length of '{}' is {}.", greeting, len);

    // 13. Write a function `change_message` that takes a mutable string reference and appends " Have fun!"
    // Your code here
    change_message(&mut message);
    println!("message: {}", message);
}
