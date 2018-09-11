# std
https://doc.rust-lang.org/std/

Crate `std` 1.0.0

The Rust Standard Library is the foundation of portable Rust software, a set of minimal and battle tested shared abstractions for the broader Rust ecosystem.

It offers:
- core types (like Vec and Option)
- library-defined operations on language primitives
- standard macros
- I/O
- multithreading
- etc.

std is available to all Rust crates by default, just as if each
one contained an `extern crate std` import at the crate root.
Therefore, std can be accessed in `use` statements through the 
path   `std`, as in `use std::env`, or in expressions through the absolute 
path `::std`, as in   `::std::env::args`.


## What is in std

1. First, std is divided into a number of focused *modules*. These modules are the bedrock upon which all of Rust is forged.

2. Second, *implicit methods on primitive types* are documented here.
  This can be a source of confusion for two reasons:

a. While *primitives are implemented by the compiler*,
   std implements methods directly on the primitive types
   (and it is the only library that does so),
   which are documented in the section on primitives.

b. std exports many *modules with the same name as primitive types*.
   These define additional items related to the 
   primitive type, but not the all-important methods.

   So, for example, there is a page for the primitive type `i32` that lists all
   the methods that can be called on 32-bit integers (very useful), and then
   there is a page for `std::i32` module that documents the constant values 
   `std::i32::MIN` and `std::i32::MAX`, which are seldom needed.

   Note the documentation for the primitives `str` and `[T]` (slice). 
   Many method calls on `String` and `Vec<T>` are actually calls to methods 
   on `str` and `[T]` respectively, via *deref coercions*.

3. std defines `The Rust Prelude`, a small collection of items, mostly traits, 
  that are imported into every module of every crate. The traits in the prelude 
  are pervasive, making the prelude documentation a good entry point to learning
  about the library.

4. std exports a number of standard macros, and lists them on this page 
  (technically, not all of the standard macros are defined by the standard lib,
  some are defined by the compiler, but they are documented here the same). 
  Like the prelude, the standard macros are imported by default into all crates.


The rest of this crate documentation is dedicated to pointing out std features.


## Containers and collections

The option and result modules define optional and error-handling types. 
The iter module defines iterator trait, which works with for to access collections.

std exposes 3 common ways to deal with *contiguous regions of memory*:
- `Vec<T>` a heap-allocated vector that is resizable at runtime.
- `[T; n]` an inline array with a fixed size at compile time.
- `[T]`dynamically sized slice into any other kind of contiguous storage

Slices can only be handled through some kind of pointer, such as:
- shared slice: `&[T]`
- mutable slice: `&mut [T]`
- owned slice: `Box<[T]>`
- string slice: `str`

UTF-8 string slice, is a primitive type, and the standard library defines many methods for it. Rust `str` are typically accessed as immutable references: `&str`. Use the owned `String` for building and mutating strings.

For converting to strings use the `format!` macro, and for converting from strings use the `FromStr` trait.

Data may be shared by placing it in a reference-counted box or the `Rc` type, and if further contained in a `Cell` or `RefCell`, may be mutated as well as hared. Likewise, in a concurrent setting it is common to pair an `Arc`, atomically reference-counted box, with a `Mutex` to get the same effect.

The collections module defines maps, sets, linked lists and other typical collection types, including the common `HashMap<K, V>`.


## Platform abstractions and I/O

Besides basic data types, the standard library is largely concerned with abstracting over differences in common platforms, most notably Windows and nix. Common types of I/O, including files, TCP, UDP, are defined in the `io`, `fs`,and `net` modules. `thread` module contains Rust's threading abstractions. `sync` contains further primitive shared memory types, including `atomic` and `mpsc`, which contains the channel types for message passing.
