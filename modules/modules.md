# Modules

Composites related
- [`result`](result/result.md) error handling with the `Result<T, E>` type.
- [`option`](option/option.md) optional value handling with `Option<T>` type.
- [`vec`](vec/vec.md) contiguous growable heap-allocated `Vec<T>`.
- [`boxed`](boxed/box.md) smart pointer for heap allocation `Box<T>`.
- [`string`](string/string.md) owned growable UTF8-encoded `String`.
- `rc` single-threaded reference-counting pointers.
- `cell` shareable mutable containers.
- `borrow`module for working with borrowed data.
- `collections` collection types.

Trait related
- `error`   Traits for working with Errors.
- `any`     enables dynamic typing of any `'static` type via reflection.
- `clone`   trait `Clone` for types that cannot be implicitly copied.
- `default` trait `Default` for types which may have meaningful default values.
- `convert` conversions between types.
- `marker`  traits and types representing basic properties of types.
- `hash`    Generic hashing support.
- `iter`    Composable external iteration.
- `ops`     Overloadable operators.
- `sync`    Useful synchronization primitives.

Env and IO related
- `io`      Traits, helpers, and type definitions for core I/O functionality.
- `os`      OS-specific functionality.
- `env`     Inspection and manipulation of the process's environment.
- `path`    Cross-platform path manipulation.
- `process` A module for working with processes.
- `fs`      Filesystem manipulation operations.
- `net`     Networking primitives for TCP/UDP communication.

Primitives related
- `i8`      The 8-bit signed integer type.
- `u8`      The 8-bit unsigned integer type.
- `i16`     The 16-bit signed integer type.
- `u16`     The 16-bit unsigned integer type.
- `i32`     The 32-bit signed integer type.
- `u32`     The 32-bit unsigned integer type.
- `i64`     The 64-bit signed integer type.
- `u64`     The 64-bit unsigned integer type.
- `i128`    The 128-bit signed integer type. [LAB]
- `u128`    The 128-bit unsigned integer type. [LAB]
- `isize`   The pointer-sized signed integer type.
- `usize`   The pointer-sized unsigned integer type.
- `f32`     f32 constants
- `f64`     f64 constants
- `char`    character type
- `slice`   dynamically-sized view into a contiguous sequence.
- `str`     Unicode string slices.
- `ptr`     Raw, unsafe pointers.

Other modules
- [`prelude`](prelude.md) The Rust Prelude
- [`mem`](mem/mem.md) Basic functions for dealing with memory.
- `cmp`     Functionality for ordering and comparison.
- `ascii`   Operations on ASCII strings and characters.
- `fmt`     Utilities for formatting and printing Strings
- `num`     Additional functionality for numerics.
- `ffi`     Utilities related to FFI bindings.
- `panic`   Panic support in the standard library
- `thread`  Native threads.
- `time`    Temporal quantification.
- `heap`    [LAB]
- `raw`     struct definitions for the layout of compiler built-in types. [LAB]
- `intrinsics` compiler intrinsics. [LAB]

