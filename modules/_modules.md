# Modules

Primitives:
`i8`      The 8-bit signed integer type.
`u8`      The 8-bit unsigned integer type.
`i16`     The 16-bit signed integer type.
`u16`     The 16-bit unsigned integer type.
`i32`     The 32-bit signed integer type.
`u32`     The 32-bit unsigned integer type.
`i64`     The 64-bit signed integer type.
`u64`     The 64-bit unsigned integer type.
`i128`    The 128-bit signed integer type. [LAB]
`u128`    The 128-bit unsigned integer type. [LAB]
`isize`   The pointer-sized signed integer type.
`usize`   The pointer-sized unsigned integer type.
`f32`     constants specific to the impl of the f32 float data type +sub-module
`f64`     constants specific to the impl of the f64 float data type +sub-module
`char`    character type.
`slice`   dynamically-sized view into a contiguous sequence.
`str`     Unicode string slices.
`ptr`     Raw, unsafe pointers. Primitive: `pointer`

Composites:
`result`      Error handling with the `Result` type.
`option`      Optional values.
`vec`         contiguous growable array type with heap-allocated contents.
`string`      A UTF-8 encoded, growable string.
`boxed`       A pointer type for heap allocation.
`rc`          Single-threaded reference-counting pointers.
`cell`        Shareable mutable containers.
`borrow`      A module for working with borrowed data.
`collections` Collection types.

Traits:
`error`   Traits for working with Errors.
`any`     enables dynamic typing of any `'static` type via reflection.
`clone`   trait `Clone` for types that cannot be implicitly copied.
`default` trait `Default` for types which may have meaningful default values.
`convert` conversions between types.
`marker`  traits and types representing basic properties of types.
`hash`    Generic hashing support.
`iter`    Composable external iteration.
`ops`     Overloadable operators.
`sync`    Useful synchronization primitives.

Other:
`prelude` The Rust Prelude.
`ascii`   Operations on ASCII strings and characters.
`fmt`     Utilities for formatting and printing Strings
`num`     Additional functionality for numerics.
`cmp`     Functionality for ordering and comparison.
`ffi`     Utilities related to FFI bindings.
`mem`     Basic functions for dealing with memory.
`panic`   Panic support in the standard library
`thread`  Native threads.
`time`    Temporal quantification.
`heap`    [LAB]
`raw`     struct definitions for the layout of compiler built-in types. [LAB]
`intrinsics` compiler intrinsics. [LAB]

Env and IO:
`io`      Traits, helpers, and type definitions for core I/O functionality.
`os`      OS-specific functionality.
`env`     Inspection and manipulation of the process's environment.
`path`    Cross-platform path manipulation.
`process` A module for working with processes.
`fs`      Filesystem manipulation operations.
`net`     Networking primitives for TCP/UDP communication.

