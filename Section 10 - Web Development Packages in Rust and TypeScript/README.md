# Section 10: Web Development Packages in Rust and TypeScript

In this section, we will explore the most commonly used packages for web development in Rust and compare them to their TypeScript counterparts. This will help you understand the Rust ecosystem and choose the right libraries when developing web applications in Rust.

## Server-side frameworks

#### Rust

In Rust, there are several popular web frameworks to choose from:

1. _Actix-Web_ - A high-performance, asynchronous web framework. It's a great choice for building fast and scalable web applications. Actix-Web is built on the Tokio async runtime and uses the Actor model for concurrency.
2. _Rocket_ - A web framework with a focus on simplicity, safety, and speed. It has a declarative syntax and automatically generates route handlers, which makes it easy to create web applications quickly.
3. _Warp_ - A lightweight web framework built on top of the Tokio async runtime. It's designed for building high-performance, composable APIs.

#### TypeScript

In TypeScript, some popular web frameworks include:

1. _Express_ - A minimal and flexible Node.js web application framework that provides a robust set of features for web and mobile applications.
2. _Koa_ - A next-generation web framework for Node.js. Koa was created by the same team behind Express and aims to be a smaller, more expressive, and more robust foundation for web applications and APIs.
3. _NestJS_ - A progressive Node.js framework for building efficient, reliable, and scalable server-side applications. NestJS uses modern JavaScript, is built with TypeScript, and combines elements of OOP, FP, and FRP.

## Front-end libraries

#### Rust

In Rust, front-end development is less common, but there are some libraries that enable building web applications with WebAssembly (Wasm):

1. _Yew_ - A modern Rust framework for creating multi-threaded front-end web applications with WebAssembly. Yew is inspired by Elm and React and brings a component-based architecture to Rust front-end development.
2. _Seed_ - A Rust front-end framework for creating fast and reliable web apps with an Elm-like architecture.

#### TypeScript

In TypeScript, there are many popular front-end libraries and frameworks, such as:

1. _React_ - A declarative, efficient, and flexible JavaScript library for building user interfaces. It allows developers to build reusable UI components and manage the state of their applications.
2. _Angular_ - A platform that makes it easy to build applications with the web. Angular combines declarative templates, dependency injection, end-to-end tooling, and integrated best practices to solve development challenges.
3. _Vue.js_ - A progressive framework for building user interfaces. Vue.js is designed to be incrementally-adoptable and focuses on the view layer, making it easy to integrate with other libraries or existing projects.

## Web development utilities

#### Rust

1. _Reqwest_ - An easy-to-use HTTP client library for Rust, built on top of the Tokio async runtime.
2. _Serde_ - A powerful serialization framework for Rust, providing a simple way to convert complex data types into JSON and other formats.

#### TypeScript

1. _Axios_ - A promise-based HTTP client for the browser and Node.js, with an easy-to-use API for making HTTP requests.
2. _Lodash_ - A modern JavaScript utility library delivering modularity, performance, and extras. Lodash provides a variety of utility functions for working with arrays, objects, strings, and more.

In this section, you have learned about some of the most popular web development packages in Rust and TypeScript. This knowledge will help you choose the right libraries and frameworks when working on web projects in Rust or TypeScript. Practice using these packages and frameworks by implementing small web projects or by contributing to open-source projects.

Now, your can practice inside of `sections10.rs` using some of the Rust web development packages mentioned above. For this exercise, you can try making an HTTP request using Reqwest and handling the response by deserializing it with Serde. Note that this file will not be a complete web application, but rather an exercise to familiarize yourself with using these libraries.

Remember that when working with external packages, you will need to add them as dependencies in your Cargo.toml file.

With the completion of this section, you have gone through the most important Rust concepts for a TypeScript developer, and you should have a good understanding of how to transition from TypeScript to Rust. Keep practicing and building projects in Rust to solidify your knowledge and become proficient in Rust development.
