# Unsafe Rust

`unsafe` allows access to these 4 features, disabling memory safety checks:
1. Dereferencing a raw pointer
2. Calling an unsafe function or method
3. Accessing or modifying a mutable static variable
4. Implementing an unsafe trait


## Dereferencing a raw pointer
- Unsafe Rust has 2 new types, similar to refs: raw pointers.
- There is immutable `*const T` and mutable `*mut T` raw pointer.
- In the context of raw pointers, immutable means that the pointer can't be
  directly assigned to after being dereferenced.
- Raw pointers are safe to create, unsafe to dereference.
- Using dereference operator on a raw pointer requires an unsafe block


## Calling an unsafe functions or methods
- Unsafe fn looks like a regular fn with `unsafe` prefixed.
- Body of unsafe fn is unsafe block.
- fn declared within `extern` block is always unsafe
- `extern` fn provides FFI


## Accessing or modifying a mutable static variable
- Global variables are called `static`
- The name of static var is in SCREAMING_SNAKE_CASE
- The type of static var must be annotated
- Only refs with `'static` lifetime can be stored in a static var.
- Accessing immutable statics is safe.
- Accessing or modifying mutable statics is unsafe.


## Implementing an unsafe trait
- Unsafe trait is declared using `unsafe` prefix.
- Implementing unsafe trait requires `unsafe` prefix.
