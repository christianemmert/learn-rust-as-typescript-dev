# Section 3: Functions

In this section, we'll learn about functions and closures in Rust and compare them with functions in TypeScript.

## Function Declaration

Both Rust and TypeScript have similar syntax for declaring functions.

#### Rust

```rust
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    greet("Alice");
}
```

#### Typescript

```
function greet(name: string) {
    console.log(`Hello, ${name}!`);
}

greet("Alice");
```

## Void Functions

In Rust and TypeScript, you can create functions that don't return a value. These are called void functions.

#### Rust

```rust
fn print_hello() {
    println!("Hello, world!");
}

fn main() {
    print_hello();
}
```

#### Typescript

```typescript
function printHello(): void {
    console.log("Hello, world!");
}

printHello();
```

## Function Return Types

In Rust, you need to specify the return type of a function using the -> syntax, while in TypeScript you use the : syntax.

#### Rust

```rust
fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    let result = add(5, 7);
    println!("Result: {}", result);
}
```

#### Typescript

```typescript
function add(x: number, y: number): number {
    return x + y;
}

const result = add(5, 7);
console.log(`Result: ${result}`);
```

## Returning Early

In Rust, you can use the return keyword to return a value early from a function, just like in TypeScript.

#### Rust

```rust
fn add_or_subtract(x: i32, y: i32, should_add: bool) -> i32 {
    if should_add {
        return x + y;
    }
    x - y
}

fn main() {
    let result = add_or_subtract(5, 7, false);
    println!("Result: {}", result);
}
```

#### Typescript

```typescript
function addOrSubtract(x: number, y: number, shouldAdd: boolean): number {
    if (shouldAdd) {
        return x + y;
    }
    return x - y;
}

const result = addOrSubtract(5, 7, false);
console.log(`Result: ${result}`);
```

## Closures

Closures are anonymous functions that can capture their environment. Rust and TypeScript both support closures.

#### Rust

```rust
fn main() {
    let add_one = |x: i32| x + 1;
    let result = add_one(5);
    println!("Result: {}", result);
}
```

#### Typescript

```typescript
const addOne = (x: number): number => x + 1;
const result = addOne(5);
console.log(`Result: ${result}`);
```

Now that you have a basic understanding of Rust's functions, practice them in the `section3.rs` file.
