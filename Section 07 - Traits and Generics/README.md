# Section 7: Traits and Generics

In this section, we'll learn about traits and generics in Rust and compare them to similar concepts in TypeScript.

## Traits

Traits in Rust are similar to interfaces in TypeScript. They define a set of methods that a type must implement.

#### Rust

```rust
trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
```

#### Typescript

Interfaces in TypeScript define a set of properties and methods that a class must implement.

```typescript
interface Summarizable {
    summarize(): string;
}

class NewsArticle implements Summarizable {
    constructor(public headline: string, public location: string, public author: string, public content: string) {}

    summarize(): string {
        return `${this.headline}, by ${this.author} (${this.location})`;
    }
}
```

## Generics

Generics in Rust are similar to generics in TypeScript. They allow you to write code that can work with multiple data types.

#### Rust

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
```

#### Typescript

Generics in TypeScript allow you to write code that can work with multiple data types.

```typescript
function largest<T>(list: T[]): T {
    let largest = list[0];

    for (const item of list) {
        if (item > largest) {
            largest = item;
        }
    }

    return largest;
}
```

Now that you have a basic understanding of Rust's traits and generics, practice it in the `section7.rs` file.
