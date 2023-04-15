// Welcome to the interactive Rust file for Section 5!
// You can use this file to practice Rust concepts as you learn them.
//
// To run this file, make sure you have Rust and Cargo installed.
// Navigate to the directory containing this file in your terminal,
// and run the command 'cargo run --bin solution5' (without quotes) to execute the code.

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

fn find_even_number(numbers: &[i32]) -> Option<i32> {
    for &number in numbers {
        if number % 2 == 0 {
            return Some(number);
        }
    }
    None
}

fn main() {
    // Exercise 1 & 2
    match divide(10.0, 2.0) {
        Ok(result) => println!("Result: {}", result),
        Err(error) => println!("Error: {}", error),
    }

    // Exercise 3 & 4
    let numbers = [1, 3, 5, 7, 9];
    match find_even_number(&numbers) {
        Some(even_number) => println!("Found an even number: {}", even_number),
        None => println!("No even numbers found"),
    }
}
