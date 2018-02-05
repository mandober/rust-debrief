# Crates


- Rust crates are on community site [crates.io](https://crates.io/)
- Rust programs and libraries are distributed as crates (packages); a binary is a crate with its complete source code (along with tests, docs, etc.)
- Rust has a phase distinction between compile-time and run-time. statically interpreted semantic rules govern the success of compilation. dynamically interpreted semantic rules govern the program's run-time behavior.
- each compilation processes a single crate with source code and produces a single crate in binary form (executable or library).
- a crate is a unit of compilation and linking, versioning, distribution and run-time loading.
- a crate contains a tree of nested module scopes, whose top level is anonymous module (from POV of paths within the module) and any item within a crate has a canonical module path denoting its location within the crate's module tree.
- every source file is a module; on the other hand module definitions can be nested in a single file.
- Each source file contains a sequence of zero or more item definitions, and may optionally begin with any number of attributes that apply to the containing module, most of which influence the behavior of the compiler. The anonymous crate module can have additional attributes that apply to the crate as a whole.
- An item is a component of a crate. Items are organized within a crate by a nested set of modules. Every crate has a single "outermost" anonymous module; all further items within the crate have paths within the module tree of the crate.
- Items are entirely determined at compile-time, generally remain fixed during execution, and may reside in ROM.
- Functions, type aliases, structs, enumerations, unions, traits and implementations may be parameterized by type. Type parameters are given as a comma-separated list of identifiers enclosed in angle brackets (<...>), after the name of the item (except for implementations, where they come directly after `impl`) and before its definition. The type parameters of an item are considered "part of the name", not part of the type of the item. 



Pull in libraries into a project i.e. declare external dependencies in `Cargo.toml` by specifying crate names and versions

```toml
[dependencies]
num = "0.1.27"
image = "0.6.1"
crossbeam = "0.2.8"
```

When compiling a program:
- `--crate-type bin` results in a binary executable
- `--crate-type lib` results in a Rust's library file `.rlib` (tells the compiler not to look for a `main` function)


`cargo build --release` produces an optimized build. Release builds run faster, but they take longer to compile, they don’t check for integer overflow, they skip `debug_assert!()`, and the stack traces they generate on panic are generally less reliable.

