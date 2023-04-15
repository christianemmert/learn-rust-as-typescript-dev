# Section 8: Modules and Packages

In this section, we'll learn about modules and packages in Rust and compare them to similar concepts in TypeScript.

## Modules

Modules in Rust allow you to organize your code into separate, reusable components.

#### Rust

```rust
// lib.rs
mod restaurant {
    pub mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {}
        }
    }
}

fn main() {
    restaurant::front_of_house::hosting::add_to_waitlist();
}
```

#### Typescript

Modules in TypeScript allow you to organize your code into separate, reusable components.

```typescript
// restaurant.ts
export namespace Restaurant {
    export namespace FrontOfHouse {
        export namespace Hosting {
            export function addToWaitlist() {}
        }
    }
}

// main.ts
import { Restaurant } from "./restaurant";
Restaurant.FrontOfHouse.Hosting.addToWaitlist();
```

## Packages

Packages in Rust are a collection of crates, which are individual binary or library targets.

#### Rust

```toml
# Cargo.toml
[dependencies]
serde = "1.0"
```

```rust
// main.rs
extern crate serde;
```

#### Typescript

Packages in TypeScript are a collection of modules that can be installed and used in your project.

```json
// package.json
{
    "dependencies": {
        "lodash": "^4.17.21"
    }
}
```

```typescript
// main.ts
import _ from "lodash";
```

Now that you have a basic understanding of Rust's modules and packages, practice it in the `section8.rs` file.
