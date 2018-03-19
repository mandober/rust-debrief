# The Rust Language

Rust is an open source programming language, whose development started in 2006 by Mozilla employee Graydon Hoare. Its development continued on GitHub by the community, steered by the Rust Project Team (a large portion of commits to the project in 2018 are from community members). Rust is dual licensed under Apache 2.0 and MIT. Rust uses `.rs` filename extension for source files and `.rlib` for libraries.

Rust v.1.0 (the first stable release) was released on May 15, 2015. Stable releases are delivered every 6 weeks, while features are developed in nightly Rust and then tested with alpha and beta releases that last 6 weeks.

Rust is a multi-paradigm programming language, employing multiple programming styles such as imperative, functional, structured, generic, etc. Rust can be used as all-level language, from low-level systems programming to shell scripting.

> _"A programming language is low level when its concepts require attention to the irrelevant"._


## Language Characteristics
- compiled
- guaranteed memory safety
- safe by default (option for unsafe)
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
- zero cost abstractions
- concurrency
- atomics
- multi-paradigms: functional, imperative, structured, generic
- module system: crates, modules
- expressive

- manual memory management (no garbage collection)
- Option type (no null)
- Result type (no exceptions)

- Typing discipline: static, strong, inferred, nominal, linear
- static
- strongly typed
- Hindley-Milner typing
- efficient type inference (type annotations are not always needed) 
- algebraic data types
- pattern matching, destructuring

- Ownership model, borrowing
- borrow checker
- lifetime concept: lifetime annotations
- resources managed through Resource Acquisition Is Initialization (RAII)
- immutability by default

- generics
- bounded parametric polymorphism
- monomorphization, static dispatch preferred
- dynamic dispatch through trait objects
- ad hoc polymorphism through the traits system
- Inheritance and polymorphism are provided by traits
- interface inheritance
- composition over inheritance

- first class functions
- higher-order functions
- block scope
- closures
- iterators
- generators (nightly)
- async features: futures, async, await (external crates)
- UTF-8 encoded strings
- control flow: if, loop, while, for, (if let, while let)
- no switch or case, no ternary constructs

- rustup for toolchain management
- cargo for project and package management
- rustdoc for documentation management
- doc comments
