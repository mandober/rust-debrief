# Unsafe Rust

- opt-in into unsafe mode using the `unsafe` keyword
- linting attribute`#![forbid(unsafe_code)]`
- `unsafe` disables memory safety checks and allows:
  1. Dereferencing a raw pointer
  2. Calling an unsafe function or method (including an intrinsic or FFn)
  3. Accessing or modifying a mutable static variable
  4. Implementing an unsafe trait
  5. Reading a field of a `union`, writing to a field of union that isn't Copy.


## Dereferencing raw pointer
- There are immutable, `*const T`, and mutable, `*mut T`, raw pointers.
- In the context of raw pointers, immutable means that the pointer can't be directly assigned to after being dereferenced.
- Raw pointers are safe to create, unsafe to dereference.
- Using dereference operator on a raw pointer requires an unsafe block.

## Calling unsafe function
- Unsafe fn looks like a regular fn with `unsafe` prefixed.
- Body of unsafe fn is unsafe block.
- fn declared within `extern` block is always unsafe
- `extern` fn provides FFI

## Modifying mutable static
- Global variables: `static`
- The type of static must be supplied
- Only refs with `'static` lifetime can be stored in a static.
- Accessing immutable statics is safe.
- Accessing or modifying mutable statics is unsafe.

## Implementing unsafe trait
- Declaring and implementing unsafe trait requires `unsafe` prefix.



## Behavior not considered unsafe
The Rust compiler does not consider the following behaviors unsafe, though they are undesirable, unexpected or erroneous.
- Deadlocks
- Leaking resources
- Exit without drop
- Pointer leaks
- Integer overflow


**Integer overflow**   
If a program contains __arithmetic overflow__, the programmer has made an error. There is a distinction between arithmetic overflow and wrapping arithmetic: the former is erroneous, the latter is intentional. When the programmer has enabled `debug_assert!` assertions (for example, by enabling a non-optimized build), implementations must insert dynamic checks that `panic` on overflow. Other kinds of builds may result in `panics` or silently wrapped values on overflow, at the implementation's discretion.

In the case of implicitly-wrapped overflow, implementations must provide well-defined (even if still considered erroneous) results by using two's complement overflow conventions. The integral types provide inherent methods to allow programmers explicitly to perform wrapping arithmetic. For example, `i32::wrapping_add` provides two's complement, wrapping addition. The standard library also provides a `Wrapping` newtype which ensures all standard arithmetic operations for `T` have wrapping semantics. See [RFC 560][560] for error conditions, rationale, and more details about integer overflow.

[560]: https://github.com/rust-lang/rfcs/blob/master/text/0560-integer-overflow.md
