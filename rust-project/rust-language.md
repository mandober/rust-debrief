# The Rust Language

Rust is a systems programming language that is fast, memory safe and multithreaded, but does not employ a garbage collector or otherwise impose significant runtime overhead.

Rust is a compiled, statically and strongly typed language with type inference that relieves programmer from annotating a fair amount of code.


https://doc.rust-lang.org/reference/crates-and-source-files.html


Crate
  - anonymous module
    - attributes
    - items
      - `mod` modules
      - `extern crate` declarations
      - `use` declarations
      - `fn` function definitions
      - `type` definitions
      - `struct` definitions
      - `enum` definitions
      - `union` definitions
      - `constant` items
      - `static` items
      - `trait` definitions
      - `impl` implementations
      - `extern` blocks



A crate is a unit of compilation and linking, as well as versioning, distribution and runtime loading. A crate contains a tree of nested module scopes. The top level of this tree is a module that is anonymous (from the point of view of paths within the module) and any item within a crate has a canonical module path denoting its location within the crate's module tree.

Every source file is a module, but not every module needs its own source file: module definitions can be nested within one file.

Each source file contains a sequence of zero or more item definitions, and may optionally begin with any number of attributes that apply to the containing module, most of which influence the behavior of the compiler. The anonymous crate module can have additional attributes that apply to the crate as a whole.

An item is a component of a crate. Items are organized within a crate by a nested set of modules. Every crate has a single "outermost" anonymous module; all further items within the crate have paths within the module tree of the crate.

Items are entirely determined at compile-time, generally remain fixed during execution, and may reside in read-only memory.

Functions, type aliases, structs, enumerations, unions, traits and implementations may be parameterized by type. Type parameters are given as a comma-separated list of identifiers enclosed in angle brackets (<...>), after the name of the item (except for implementations, where they come directly after impl) and before its definition. The type parameters of an item are considered "part of the name", not part of the type of the item. 


