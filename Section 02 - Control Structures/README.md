# Section 3: Control Flow

In this section, we'll explore control flow constructs in Rust and compare them with their TypeScript counterparts.

## If and If-Else Expressions

Both Rust and TypeScript have similar syntax for `if` and `if-else` expressions. One notable difference is that Rust does not have a ternary operator like TypeScript.

#### Rust

```rust
let x = 5;

if x > 3 {
    println!("x is greater than 3");
} else if x > 5 {
    println!("x is greater than 5 but not greater than 10");
} else {
    println!("x is not greater than 3");
}
```

#### Typescript

```typescript
const x = 5;

if (x > 10) {
    console.log("x is greater than 10");
} else if (x > 5) {
    console.log("x is greater than 5 but not greater than 10");
} else {
    console.log("x is not greater than 5");
}

// Using the ternary operator in TypeScript
const message = x > 3 ? "x is greater than 3" : "x is not greater than 3";
```

## Loops

Both Rust and TypeScript have for, while, and loop constructs for iteration.

#### Rust

```rust
// Using a for loop with a range
for i in 1..4 {
    println!("i: {}", i);
}

// Using a while loop
let mut count = 0;
while count < 5 {
    println!("count: {}", count);
    count += 1;
}

// Using a loop construct (infinite loop)
let mut counter = 0;
loop {
    counter += 1;
    if counter == 10 {
        break;
    }
}
```

#### Typescript

```typescript
// Using a for loop
for (let i = 1; i < 4; i++) {
    console.log(`i: ${i}`);
}

// Using a while loop
let count = 0;
while (count < 5) {
    console.log(`count: ${count}`);
    count++;
}

// Using a for loop as an infinite loop
let counter = 0;
for (;;) {
    counter++;
    if (counter === 10) {
        break;
    }
}
```

Now that you have a basic understanding of Rust's control flow constructs, practice them in the section2.rs file.
