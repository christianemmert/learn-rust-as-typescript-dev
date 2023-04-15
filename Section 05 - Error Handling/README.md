# Section 5: Error Handling

In this section, we'll learn about error handling in Rust and compare it with similar concepts in TypeScript.

Error handling in Rust is done using the `Result` and `Option` types, while TypeScript typically uses exceptions and `try-catch` blocks.

Rust has two types to handle errors and represent the possibility of failure: `Result` and `Option`.

## Result

`Result` is an enum with two variants: `Ok` and `Err`. It is used to represent the outcome of a computation that might fail. The `Ok` variant holds the successful result, and the `Err` variant holds an error value.

#### Rust

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

You can use pattern matching to handle the different outcomes.

#### Rust

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    match divide(10.0, 2.0) {
        Ok(result) => println!("Result: {}", result),
        Err(error) => println!("Error: {}", error),
    }
}
```

## Option

Option is another enum with two variants: Some and None. It is used to represent a value that may or may not be present. The Some variant holds the value if it exists, and the None variant represents the absence of a value.

#### Rust

```rust
enum Option<T> {
    Some(T),
    None,
}
```

You can also use pattern matching to handle the different outcomes with `Option`.

#### Rust

```rust
fn find_even_number(numbers: &[i32]) -> Option<i32> {
    for &number in numbers {
        if number % 2 == 0 {
            return Some(number);
        }
    }
    None
}

fn main() {
    let numbers = [1, 3, 5, 7, 9];
    match find_even_number(&numbers) {
        Some(even_number) => println!("Found an even number: {}", even_number),
        None => println!("No even numbers found"),
    }
}
```

## Panicking

In Rust, panicking is used to handle unrecoverable errors that should cause the program to terminate. When a panic occurs, the program starts unwinding the stack, running destructors and cleaning up resources before exiting. Panics can be triggered explicitly with the `panic!` macro or implicitly when certain operations fail, such as an out-of-bounds array access.

Panicking should be used sparingly, as it is less flexible than returning `Result` or `Option`. However, it can be useful in situations where an error should not be recoverable, or when an invariant or assumption is violated.

#### Rust

```rust
fn main() {
    let v = vec![1, 2, 3];
    // This will cause a panic because the index is out of bounds.
    let element = v[5];
}
```

You can also use the `unwrap` method on `Result` and `Option` types, which will return the value if it's `Ok` or `Some`, and cause a panic if it's `Err` or `None`.

#### Rust

```rust
fn main() {
    let x: Result<i32, &str> = Err("An error occurred");
    let y = x.unwrap(); // This will cause a panic since x is Err.
}
```

It's often better to use methods like `unwrap_or` or `unwrap_or_default` to provide a default value or handle the error case without panicking.

#### Typescript

```typescript
function divide(a: number, b: number): number {
    if (b === 0) {
        throw new Error("Cannot divide by zero");
    }
    return a / b;
}

try {
    const result = divide(10, 2);
    console.log(`Result: ${result}`);
} catch (error) {
    console.log(`Error: ${error.message}`);
}
```

Now that you have a better understanding of Rust's error handling using Result, Option types, and panicking, practice them in the `section5.rs` file.
