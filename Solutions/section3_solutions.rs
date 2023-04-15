// Welcome to the interactive Rust file for Section 3!
// You can use this file to practice Rust concepts as you learn them.
//
// To run this file, make sure you have Rust and Cargo installed.
// Navigate to the directory containing this file in your terminal,
// and run the command 'cargo run --bin solution3' (without quotes) to execute the code.

fn sum(x: i32, y: i32) -> i32 {
    x + y
}

fn is_even(x: i32) -> bool {
    x % 2 == 0
}

fn print_hello() {
    println!("Hello, world!");
}

fn main() {
    // 1. Define a function `sum` that takes two i32 parameters and returns their sum as an i32.
    // Solution: See the function `sum` above.

    // 2. Call the `sum` function with two numbers, and print the result.
    let sum_result = sum(5, 7);
    println!("Sum: {}", sum_result);

    // 3. Define a function `is_even` that takes an i32 parameter and returns a bool indicating whether the number is even.
    // Solution: See the function `is_even` above.

    // 4. Call the `is_even` function with a number, and print the result.
    let is_even_result = is_even(5);
    println!("Is even? {}", is_even_result);

    // 5. Define a void function `print_hello` that prints "Hello, world!".
    // Solution: See the function `print_hello` above.

    // 6. Call the `print_hello` function.
    print_hello();

    // 7. Define a closure `multiply_by_two` that takes an i32 parameter and returns the result of multiplying it by 2.
    let multiply_by_two = |x: i32| x * 2;

    // 8. Call the `multiply_by_two` closure with a number, and print the result.
    let multiply_result = multiply_by_two(5);
    println!("Multiply by two: {}", multiply_result);
}
