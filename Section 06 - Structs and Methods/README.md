# Section 6: Structs and Methods

In this section, we'll learn about structs and methods in Rust and compare them to similar concepts in TypeScript.

## Structs

Structs in Rust are similar to classes in TypeScript. They are used to create custom data types by grouping together related pieces of data.

#### Rust

```rust
struct Point {
    x: f64,
    y: f64,
}
```

## Methods

Methods in Rust are similar to methods in TypeScript. They are defined within an impl block for a struct and can access the struct's fields.

#### Rust

```rust
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
    let point = Point { x: 3.0, y: 4.0 };
    println!("Distance from origin: {}", point.distance_from_origin());
}
```

#### Typescript

In TypeScript, classes are used to create custom data types by grouping together related pieces of data and methods.

```typescript
class Point {
    x: number;
    y: number;

    constructor(x: number, y: number) {
        this.x = x;
        this.y = y;
    }

    distanceFromOrigin(): number {
        return Math.sqrt(this.x ** 2 + this.y ** 2);
    }
}

const point = new Point(3, 4);
console.log(`Distance from origin: ${point.distanceFromOrigin()}`);
```

Now that you have a basic understanding of Rust's structs and methods, practice it in the `section6.rs` file.
