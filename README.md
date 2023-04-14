# Learning Rust as a Typescript Developer

## Introduction

Welcome to "Learning Rust as a TypeScript Developer"! This repository aims to help TypeScript developers learn Rust by comparing Rust concepts with their TypeScript counterparts and providing practical exercises.

I haven't found such a resource so I decided to create one by myself.

Note that this is not an official Rust resource.

## Why Rust?

Rust is a systems programming language that focuses on safety, speed, and concurrency. Some of the advantages Rust offers over other languages include:

-   Memory safety without a garbage collector, using its unique ownership system.
-   Strong static typing and type inference, preventing many errors at compile-time.
-   High performance, similar to C and C++.
-   Fearless concurrency, making it easier to write efficient multi-threaded code.

Although Rust and TypeScript are different languages with unique features, both are statically typed and share some similarities in syntax and concepts. This repository will help you leverage your TypeScript knowledge to learn Rust effectively.

In the following sections, we will explore Rust concepts alongside their TypeScript counterparts and provide interactive exercises to reinforce your learning. Good luck!

## Prerequisites

1. Install Rust and cargo package manager via

```
curl https://sh.rustup.rs -sSf | sh
```

## How to use

Work through the `README.md` of a section. After that start practicing.
Each section includes a `section<SECTION_NUMBER>.rs` file with a different set of tasks to play around.

If you think you have filled all missing parts `// your code here`, run via

```
cargo run --bin section<SECTION_NUMBER>
```

If you feel stuck for every exercise there is a solution inside of `/Solutions`. Run the solution via

```
cargo run --bin solution<SECTION_NUMBER>
```
