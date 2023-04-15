// Welcome to the interactive Rust file for Section 6!
// You can use this file to practice Rust concepts as you learn them.
//
// To run this file, make sure you have Rust and Cargo installed.
// Navigate to the directory containing this file in your terminal,
// and run the command 'cargo run --bin solution6' (without quotes) to execute the code.

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    // Exercise 1 & 2
    let point = Point { x: 3.0, y: 4.0 };

    // Exercise 3 & 4
    println!("Distance from origin: {}", point.distance_from_origin());
}
