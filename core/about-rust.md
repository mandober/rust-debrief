# The Rust Programming Language


Rust is a all-level, general-purposes, multi-paradigm programming language providing safety, speed, concurrency and control without garbage collection.


## Info
- Rust was started in 2006 by Graydon Hoare working at Mozilla
- Mozilla began sponsoring Rust in 2009
- First official presentation in 2010
- Release v.1.0.0 on May 15 2015
- Development by the community, on GitHub, steered by the Rust Project Team
- Open source, dual licensed under Apache 2.0 and MIT.
- Filename extension for source files:`.rs`, libraries: `.rlib`
- Stable releases delivered every 6 weeks
- Features developed in nightly Rust and tested with alpha/beta for 6 weeks.


> _"A programming language is low level when its concepts require attention to the irrelevant"._


## Specialty
- Rust = Safety + Performance + Expressiveness + Control + Concurrency (- GC)
- Rust is the only language that makes concurrent programming memory safe without GC, with performance rivaling that of C, C++
- Rust is the only language that provides safety with expressiveness without sacrificing control or performance.


- Low overhead, small runtime
- Zero Cost Abstractions (ZCA)
- Choose Your Guarantees (CYG)
- Pay only what you use
- Control AND safety: Provides maximum control while also providing maximum safety




## Features
- Compiled: compiling to multiple targets and formats
- Self-hosting compiler written in Rust, `rustc`, using LLVM as the back-end
- Safe
- Concurrent, parallel, system threads
- UTF-8 encoded strings
- FFI
- Performance is comparable to C++
- Zero Cost Abstractions (ZCA)
- Choose Your Guarantees (CYG)
- Pay only what you use

## Guarantees
- guaranteed memory safety
- no data races
- no dangling or wild or null pointers
- safe by default (optional unsafety)

## !Features (absent concepts)
- no GC
- no null
- no exceptions
- not oo
- no classes, no fn overloading



- The standard library, can be avoided
- small runtime code
- built-in unit and integrated testing, assertions via macros
- operator overloading
- meta elements: macros, attributes
- favors stack allocation
- lazy evaluation
- concurrency
- atomics
- multi-paradigms: functional, imperative, structured, generic
- module system: crates, modules
- expressive
- manual memory management (no garbage collection)
- Option type (no null)
- Result type (no exceptions)
- Typing discipline: static, strong, inferred, linear
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
- control flow: if, loop, while, for, (if let, while let)
- no switch or case, no ternary constructs

- rustup for toolchain management
- cargo for project and package management
- rustdoc for documentation management
- doc comments



## Influences
- Abstract Machine Model : C
- Data types : C, SML, OCaml, Lisp, Limbo
- Optional Bindings : Swift
- Hygienic Macros : Scheme
- Functional Programming : Haskell, OCaml, F#
- Attributes : ECMA-335
- Memory Model and Memory Management : C++, ML Kit, Cyclone
- Type Classes : Haskell
- Crate : Assembly in the ECMA-335 CLI model
- Channels and Concurrency : Newsqueak, Alef, Limbo
- Message passing and Thread failure : Erlang
