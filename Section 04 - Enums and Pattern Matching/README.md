# Section 4: Structs and Enums

In this section, we'll learn about structs and enums in Rust and compare them with similar concepts in TypeScript.

## Structs

Structs are used to create custom data types in Rust. In TypeScript, you can use interfaces or classes to achieve similar functionality.

#### Rust

```rust
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let alice = Person {
        name: String::from("Alice"),
        age: 30,
    };

    println!("{} is {} years old.", alice.name, alice.age);
}
```

#### Typescript

```typescript
interface Person {
    name: string;
    age: number;
}

const alice: Person = {
    name: "Alice",
    age: 30,
};

console.log(`${alice.name} is ${alice.age} years old.`);
```

## Enums

Enums allow you to define a type that represents one of a few possible variants. In TypeScript, you can use enums or union types to achieve similar functionality.

#### Rust

```rust
enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    let direction = Direction::North;

    match direction {
        Direction::North => println!("Heading North!"),
        Direction::South => println!("Heading South!"),
        Direction::East => println!("Heading East!"),
        Direction::West => println!("Heading West!"),
    }
}
```

#### Typescript

```typescript
enum Direction {
    North,
    South,
    East,
    West,
}

const direction: Direction = Direction.North;

switch (direction) {
    case Direction.North:
        console.log("Heading North!");
        break;
    case Direction.South:
        console.log("Heading South!");
        break;
    case Direction.East:
        console.log("Heading East!");
        break;
    case Direction.West:
        console.log("Heading West!");
        break;
}
```

Now that you have a basic understanding of Rust's structs and enums, practice them in the `section4.rs` file.
