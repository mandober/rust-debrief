# The Rust Project

Rust is a compiled, statically and strongly typed language with type inference that relieves programmer from annotating a fair amount of code.


* Language
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
* Tools
  - [cargo](https://github.com/rust-lang/cargo) package manager, test runner, etc.
  - [rustup](https://github.com/rust-lang-nursery/rustup.rs) managing rust installations and toolchains
  - [rustdoc](https://github.com/rust-lang/rust/blob/master/src/doc/rustdoc/src/what-is-rustdoc.md) generating docs from source comments
  - etc.
* Third-party crates
  - crate repository at [crates.io](https://crates.io/)
