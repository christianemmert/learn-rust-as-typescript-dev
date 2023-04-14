# Section 2: Basic Syntax and Data Types

In this section, we'll cover basic syntax and data types in Rust, comparing them to TypeScript.

## Variables and Constants

In Rust, variables are immutable by default, which means their values cannot be changed once assigned. To create a mutable variable, use the `mut` keyword. In TypeScript, you use `const` for constants and `let` for mutable variables.

### Rust

```rust
let x = 5; // immutable variable
let mut y = 6; // mutable variable
y = 7; // can modify the value of y
```

### Typescript

```typescript
const x = 5; // constant
let y = 6; // mutable variable
y = 7; // can modify the value of y
```

## Data Types

Rust has several built-in data types, including integers, floating-point numbers, Booleans, and characters.

### Integers

Rust has signed and unsigned integer types with different bit lengths: i8, i16, i32, i64, i128, u8, u16, u32, u64, and u128. The default integer type is i32.

```rust
let a: i32 = -42;
let b: u32 = 42;
```

In TypeScript, there is only one number type for both integers and floating-point numbers.

```typescript
const a: number = -42;
const b: number = 42;
```

### Floating-point numbers

Rust has two floating-point number types: f32 and f64. The default type is f64.

### Booleans

Rust's boolean type is bool, just like in TypeScript.

```rust
let is_active: bool = true;
```

```typescript
const isActive: boolean = true;
```

### Characters

Rust has a char type for Unicode scalar values. In TypeScript, characters are represented using the string type with a single character.

```rust
let letter: char = 'A';
let emoji: char = '😀';
```

```typescript
const letter: string = "A";
const emoji: string = "😀";
```

### Strings and &str

In Rust, there are two string types: String and &str. The String type is a growable, mutable, owned, UTF-8 encoded string, while &str is a string slice, which is an immutable reference to a part of a String.

```rust
let s: &str = "hello"; // string slice
let mut t: String = String::from("world"); // String type
t.push_str(", Rust!"); // modifying the String
```

In TypeScript, the string type represents all strings, and they are immutable.

```typescript
const s: string = "hello";
let t: string = "world";
t = t + ", TypeScript!";
```

Now that you have a basic understanding of Rust's syntax and data types, practice them in the section2.rs.
