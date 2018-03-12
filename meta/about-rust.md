# Rust Language

Rust is an open source programming language, whose development started in 2006 by Mozilla employee Graydon Hoare. Its development continued on GitHub by the Rust project developers - a large portion of current commits to the project are from community members. Rust is dual licensed under Apache 2.0 and MIT. Rust uses `.rs` filename extension for source files, `.rlib` for libraries.

Rust 1.0, the first stable release, was released on May 15, 2015. Stable releases are delivered every six weeks, while features are developed in nightly Rust and then tested with alpha and beta releases that last six weeks.


- compiled
- guaranteed memory safety
- safe by default (optin for unsafe)
- no data races
- no dangling, wild pointers, null pointers
- performance is comparable to C++
- compiling to wasm, etc.
- self-hosting compiler written in Rust, rustc, using LLVM as back end
- foreign function interface (FFI)
- built-in unit and integrated testing
- assertions via macros
- operator overloading
- meta elements: macros, attributes
- favors stack allocation
- lazy evaluation
- zero cost abstactions
- concurrency
- atomics
- multi style paradigms: functional, imperative, structured, generic
- module system: crates, modules
- expressive

- manual memory management (no garbage collection)
- Option type (no null)
- Result type (no exceptions)

- Typing discipline: static, strong, inferred, nominal, linear
- static
- strongly typed
- Hindley–Milner typing
- efficient type inference (type annotations are not alwasy needed) 
- algebraic data types
- pattern matching, destructuring

- Ownership model, borrowing
- borrow checker
- lifetime concept: lifetime annotations
- resources managed through Resource Acquisition Is Initialization (RAII)
- immutablility by default

- generics
- monomorphization, static dispatch prefered
- dynamic dispatch through trait objects
- ad hoc polymorphism through the traits system
- Inheritance and polymorphism are provided by traits
- interface inheritance
- composition over inheritance

- first class functions
- higher-order functions
- closures
- iterators
- generators (nightly)
- asynchronisity: futures, async, await (external crates)
- UTF-8 encoded strings
- control flow: if, loop, while, for, (if let, while let)
- no case, switch, ternary constructa
- block scope

- rustup for toolchain management
- cargo for project and package management
- rustdoc for documentation management
- doc comments
