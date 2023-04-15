// Welcome to the interactive Rust file for Section 4!
// You can use this file to practice Rust concepts as you learn them.
//
// To run this file, make sure you have Rust and Cargo installed.
// Navigate to the directory containing this file in your terminal,
// and run the command 'cargo run --bin solution4' (without quotes) to execute the code.

struct Rectangle {
    width: f32,
    height: f32,
}

enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    // 1. Define a struct `Rectangle` with two fields: width and height (both of type f32).
    // Solution: See the struct `Rectangle` above.

    // 2. Create an instance of the `Rectangle` struct with width 5.0 and height 10.0.
    let rect = Rectangle {
        width: 5.0,
        height: 10.0,
    };

    // 3. Print the width and height of the rectangle.
    println!("Rectangle width: {}, height: {}", rect.width, rect.height);

    // 4. Define an enum `Color` with variants Red, Green, and Blue.
    // Solution: See the enum `Color` above.

    // 5. Create a variable `favorite_color` with the value `Color::Green`.
    let favorite_color = Color::Green;

    // 6. Use a match expression to print a message based on the value of `favorite_color`.
    match favorite_color {
        Color::Red => println!("Your favorite color is Red!"),
        Color::Green => println!("Your favorite color is Green!"),
        Color::Blue => println!("Your favorite color is Blue!"),
    }
}
