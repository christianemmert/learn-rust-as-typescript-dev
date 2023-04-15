# Section 1: Variables, Data Types and Borrowing

In this section, we'll cover variables, data types, and borrowing in Rust, comparing them to TypeScript.

## Variables and Constants

In Rust, variables are immutable by default, which means their values cannot be changed once assigned. To create a mutable variable, use the `mut` keyword. In TypeScript, you use `const` for constants and `let` for mutable variables.

#### Rust

```rust
let x = 5; // immutable variable
let mut y = 6; // mutable variable
y = 7; // can modify the value of y
```

#### Typescript

```typescript
const x = 5; // constant
let y = 6; // mutable variable
y = 7; // can modify the value of y
```

## Data Types

Rust has several built-in data types, including integers, floating-point numbers, Booleans, and characters.

### Integers

Rust has signed and unsigned integer types with different bit lengths: i8, i16, i32, i64, i128, u8, u16, u32, u64, and u128. The default integer type is i32.

#### Rust

```rust
let a: i32 = -42;
let b: u32 = 42;
```

In TypeScript, there is only one number type for both integers and floating-point numbers.

#### Typescript

```typescript
const a: number = -42;
const b: number = 42;
```

### Floating-point numbers

Rust has two floating-point number types: f32 and f64. The default type is f64.

### Booleans

Rust's boolean type is bool, just like in TypeScript.

#### Rust

```rust
let is_active: bool = true;
```

#### Typescript

```typescript
const isActive: boolean = true;
```

### Characters

Rust has a char type for Unicode scalar values. In TypeScript, characters are represented using the string type with a single character.

#### Rust

```rust
let letter: char = 'A';
let emoji: char = '😀';
```

#### Typescript

```typescript
const letter: string = "A";
const emoji: string = "😀";
```

### Strings and &str

In Rust, there are two string types: String and &str. The String type is a growable, mutable, owned, UTF-8 encoded string, while &str is a string slice, which is an immutable reference to a part of a String.

#### Rust

```rust
let s: &str = "hello"; // string slice
let mut t: String = String::from("world"); // String type
t.push_str(", Rust!"); // modifying the String
```

In TypeScript, the string type represents all strings, and they are immutable.

#### Typescript

```typescript
const s: string = "hello";
let t: string = "world";
t = t + ", TypeScript!";
```

### Tuple

Tuples in Rust are fixed-size collections of elements with different types. In TypeScript, tuples are used to represent an array where the type of a fixed number of elements is known, but the length of the array may vary.

#### Rust

```rust
let tuple: (i32, f64, bool) = (42, 3.14, true);
let (x, y, z) = tuple; // destructuring
```

#### Typescript

```typescript
const tuple: [number, string, boolean] = [42, "Hello", true];
const [x, y, z] = tuple; // destructuring
```

### Array

Arrays in Rust are fixed-size collections of elements with the same type. In TypeScript, arrays are dynamically-sized and can hold elements of the same type.

#### Rust

```rust
let arr: [i32; 5] = [1, 2, 3, 4, 5];
let first_element = arr[0]; // access element by index
```

#### Typescript

```typescript
const arr: number[] = [1, 2, 3, 4, 5];
const firstElement = arr[0]; // access element by index
```

## Borrowing

Borrowing in Rust allows you to have multiple references to the same value without violating ownership rules. There are two types of borrowing: shared and mutable.

### Shared Borrow

Shared borrows allow multiple read-only references to the same value.

#### Rust

```rust
fn main() {
    let s = String::from("hello");
    let len = calculate_length(&s);
    println!("The length of '{}' is {}.", s, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

### Mutable Borrow

Mutable borrows allow a single mutable reference to the same value.

#### Rust

```rust
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("The new string is '{}'.", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

### Ownership

Ownership is a central concept in Rust. It ensures memory safety without the need for a garbage collector.

Rules of Ownership

1. Each value in Rust has a single owner.
2. When the owner goes out of scope, the value will be dropped.
3. While a value is borrowed, the owner is not allowed to modify it.

Now that you have a basic understanding of Rust's syntax and data types, practice them in the `section1.rs`.
