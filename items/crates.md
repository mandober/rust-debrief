# Crates

- Rust crates community site: [crates.io](https://crates.io/)
- Rust programs and libraries are distributed as packages called crates
- a crate is a unit of compilation and linking, versioning, distribution and run-time loading.
- each compilation processes a single crate with source code and produces a single crate in binary form (executable or library).
- a crate contains a tree of nested module scopes, whose top level is anonymous module (from POV of paths within the module) and any item within a crate has a canonical module path denoting its location within the crate's module tree.




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

