# About Rust

- systems programming language; 
- multithreaded
- memory safety
- explicitness over implicitness
- no significant runtime overhead
- no pointer troubles
- choose your guarantees
- safe and unsafe
- FFI
- phase distinction between compile-time and run-time.
- statically interpreted semantic rules govern the success of compilation.
- dynamically interpreted semantic rules govern the program's run-time behavior.
- strongly typed with type inference
- pattern matching
- code organization with traits
- crates (packages), package manager: cargo


- no garbage collector
- no `NULL`
- no classes
- no exceptions


Rust is a compiled, statically and strongly typed language with type inference that relieves programmer from annotating a fair amount of code.

Perhaps a way to classify the whole of the Rust ecosystem would be a separation to the language itself and its libraries, the tools and the third-party packages.


* **Language**
  - The compiler (primitives, lang items, type safety, borrowck, etc.)
  - Base Libraries
    * The Rust Core Library, `core`
    * The Rust Core Allocation and Collections Library, `alloc`
    * The Unicode library, `std_unicode`
    * Procedural Macros Support Library, `proc_macro`
    * The Rust Standard Library, `std`
      - Methods on primitives
      - Modules: own modules and reexported modules from base libraries
      - Standard macros
      - The Rust Prelude
* **Tools**
  - [cargo](https://github.com/rust-lang/cargo) package manager, test runner, etc.
  - [rustup](https://github.com/rust-lang-nursery/rustup.rs) managing rust installations and toolchains
  - [rustdoc](https://github.com/rust-lang/rust/blob/master/src/doc/rustdoc/src/what-is-rustdoc.md) generating docs from source comments
  - etc.
* **Third-party crates**
  - crate repository at [crates.io](https://crates.io/)



## The Rust Language
The Rust language is implemented by the compiler and the base packages (Rust's source code packages are called "crates"). Rust's compiler is written in Rust and also organized in several crates, enforces type safety rules and defines primitive types of the laguage.

The base crates `core`, `alloc`, `std_unicode` and `proc_macro` provide other necessary functionality made accessible through the `std` crate, also known as *The Rust Standard Library*. The `std` crate/library defines methods on the primitive types and by reexporting all the functionality from the other base crates, it becomes the only base crate that needs to be referred to (e.g. in `use` statements), when and if needed.


## Tools
The package manager is called [cargo](https://github.com/rust-lang/cargo), it manages installation of crates hosted at [crates.io](https://crates.io/). It integrates different services providing project management, documentation  generation, running test, etc. 

The tool for managing multiple Rust installations and toolchains is called [rustup](https://github.com/rust-lang-nursery/rustup.rs). It enables easy switching between stable, beta, and nightly compilers and keeps things updated. It makes cross-compiling simpler.


## Release channels
There are 3 channels for Rust releases: [Nightly](https://doc.rust-lang.org/nightly/), [Beta](https://doc.rust-lang.org/beta/) and [Stable](https://doc.rust-lang.org/stable). New nightly releases are created once a day. Every 6 weeks, the latest nightly release is promoted to Beta. At that point, it will only receive patches to fix serious errors. 6 weeks later, the beta is promoted to Stable, and becomes the next (minor i.e. 1.x) release.


## License
Rust is primarily distributed under the terms of both the **MIT** license and the **Apache License (Version 2.0)**, with portions covered by various **BSD-like** licenses.