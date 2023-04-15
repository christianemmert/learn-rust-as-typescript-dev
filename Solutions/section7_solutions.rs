// Welcome to the interactive Rust file for Section 7!
// You can use this file to practice Rust concepts as you learn them.
//
// To run this file, make sure you have Rust and Cargo installed.
// Navigate to the directory containing this file in your terminal,
// and run the command 'cargo run --bin solution7' (without quotes) to execute the code.

// Exercise 1
trait Summarizable {
    fn summarize(&self) -> String;
}

// Exercise 2
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

// Exercise 3
impl Summarizable for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

// Exercise 4
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Rust is awesome!"),
        location: String::from("Internet"),
        author: String::from("Jane Doe"),
        content: String::from("Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety."),
    };
    println!("NewsArticle: {}", article.summarize());

    let numbers = [1, 50, 10, 100, 25];
    println!("The largest number is: {}", largest(&numbers));

    let chars = ['a', 'y', 'b', 'Z', 'X'];
    println!("The largest char is: {}", largest(&chars));
}
