# Crates

- The Rust core library, `core`
- The Rust core allocation and collections library, `alloc`
- Procedural macros support library, `proc_macro`
- The Unicode library, `std_unicode` (experimental)
- The Rust Standard Library, `std`:
   * Methods on primitives
   * Modules
   * Standard macros
   * The Rust Prelude



## The Rust Standard Library

[The Rust Standard Library](https://doc.rust-lang.org/std/index.html), `std`, is the foundation of portable Rust software, a set of minimal and battle-tested shared abstractions for the broader [Rust ecosystem](https://crates.io/). It offers core types, library-defined operations on language primitives, standard macros, I/O and multithreading, etc. Language items from other libraries are imported by the standard library and exported for general use - all language items from these basic crates are accessible through the standard library. The std is available to all Rust crates by default, just as if each one contained an `extern crate std` import at the crate root. 

Therefore the standard library can be accessed in `use` statements through the 
path `  std`, as `use  std::env`, or in expressions through the absolute 
path `::std`, as in `::std::env::args`.

The standard library is divided into a number of focused **modules**.
The standard library contains documentation of implicit methods on primitive types. Namely, while primitive types are implemented by the compiler, it is the std that implements **methods on the primitives**. 

It exports many **modules with the same name as primitive types**, which define additional items related to primitives.


---

> The `core` and `alloc` libraries are not intended for general usage, but rather as the building blocks of other libraries. Language items in these libraries are *reexported* through the standard library i.e. there are identical modules in core/alloc and std e.g., `core::i32` and `std::i32`; language items should not be used through their originating libraries, but through the standard library e.g. use `std::i32` rather than `core::i32`.


## The Rust Core Library
[The Rust Core Library](https://doc.rust-lang.org/core/index.html), `core`, is the dependency-free foundation of The Rust Standard Library. It is the portable glue between the language and its libraries, defining the intrinsic and primitive building blocks of all Rust code. It links to no upstream libraries, no system libraries, and no libc. The core library is minimal: it isn't even aware of heap allocation, nor does it provide concurrency or I/O. These things require platform integration, and this library is platform-agnostic.


## The Rust core allocation and collections library
[The Rust core allocation and collections library](https://doc.rust-lang.org/alloc/index.html), `alloc`crate, provides smart pointers and collections for managing heap-allocated values.


## The Unicode Library
[The Unicode Library](https://doc.rust-lang.org/std_unicode/index.html), `std_unicode`, crate provides a collection of Unicode-related functionality, including decompositions, conversions, etc, and provides traits implementing these functions for the `char` and `str` types. The functionality included is only that which is necessary to provide for basic string-related manipulations. This crate does not yet aim to provide a full set of Unicode tables. This is a nightly-only experimental API, [unicode #27783](https://github.com/rust-lang/rust/issues/27783).


## The Collections library
[The Collections library] (https://doc.rust-lang.org/collections/index.html), `collections`, has been deprecated since Rust version 1.20.0. It was moved to `alloc` library; it is also accessible through the std library.


## The Procedural macros library
[The Procedural macros library](https://doc.rust-lang.org/proc_macro/index.html), `proc_macro`, is a support library for macro authors when defining new macros. It provides the types consumed in the interfaces of procedurally defined macro definitions. Currently the primary use of this crate is to provide the ability to define new custom derive modes.


## Links to documentation on doc.rust-lang.org
[`core`](https://doc.rust-lang.org/core/index.html)
[`std`](https://doc.rust-lang.org/std/)
[`std_unicode`](https://doc.rust-lang.org/std_unicode/index.html)
[`collections`](https://doc.rust-lang.org/collections/index.html)
[`alloc`](https://doc.rust-lang.org/alloc/index.html)
[`proc_macro`](https://doc.rust-lang.org/proc_macro/index.html)
