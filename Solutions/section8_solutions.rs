// Welcome to the interactive Rust file for Section 8!
// You can use this file to practice Rust concepts as you learn them.
//
// To run this file, make sure you have Rust and Cargo installed.
// Navigate to the directory containing this file in your terminal,
// and run the command 'cargo run --bin solution8' (without quotes) to execute the code.

// Exercise 1
mod library {
    // Exercise 2
    pub mod books {
        // Exercise 3
        pub fn borrow_book() {
            println!("Borrowing a book...");
        }
    }
}

// Exercise 4
fn main() {
    library::books::borrow_book();
}
