# Static items

- static item represents a precise memory location in the program.
- static is never inlined at the usage site, and all references to it refer to the same memory location.
- statics have static lifetime
- statics may be in ROM, if they don't have interior mutability.
- they may contain interior mutability through `UnsafeCell`.
- Rust 1.22: types that impl Drop are now allowed in const and static types
- generally, constants should be preferred over statics, unless large data is being stored or single-address and mutability are required.
- access to static is safe, with restrictions:
  - they must not have any destructors
  - types of static values must ascribe to `Sync`
  - may not refer to other statics by value, only by reference.
  - constants cannot refer to statics.
- statics can be mutable: `unsafe` block required to access a mut static; they have the same restrictions as immutable statics, except `Sync` isn't required.



## Mutable statics

One of Rust's goals is to make concurrency bugs hard to run into, and this is obviously a very large source of race conditions or other bugs. For this reason, an `unsafe` block is required when either reading or writing a mutable static variable. Care should be taken to ensure that modifications to a mutable static are safe with respect to other threads running in the same process.

Mutable statics are still very useful, however. They can be used with C libraries and can also be bound from C libraries (in an `extern` block).


```rust
static mut LEVELS: u32 = 0;

// This violates the idea of no shared state, and this doesn't internally
// protect against races, so this function is `unsafe`
unsafe fn bump_levels_unsafe1() -> u32 {
    let ret = LEVELS;
    LEVELS += 1;
    return ret;
}

// Assuming that we have an atomic_add function which returns the old value,
// this function is "safe" but the meaning of the return value may not be what
// callers expect, so it's still marked as `unsafe`
unsafe fn bump_levels_unsafe2() -> u32 {
    return atomic_add(&mut LEVELS, 1);
}
```

Mutable statics have the same restrictions as normal statics, except
that the type of the value is not required to ascribe to `Sync`.


## `'static` lifetime elision

Both constant and static declarations of reference types have implicit static lifetimes unless an explicit lifetime is specified. As such, the constant declarations involving `'static` above may be written without the lifetimes.

Returning to our previous example:

```rust
const BIT1: u32 = 1 << 0;
const BIT2: u32 = 1 << 1;

const BITS: [u32; 2] = [BIT1, BIT2];
const STRING: &str = "bitstring";

struct BitsNStrings<'a> {
    mybits: [u32; 2],
    mystring: &'a str,
}

const BITS_N_STRINGS: BitsNStrings = BitsNStrings {
    mybits: BITS,
    mystring: STRING,
};
```

Note that if the static or const items include function or closure references, which themselves include references, the compiler will first try the standard elision rules.
Discussion: [nomicon](https://doc.rust-lang.org/nomicon/lifetime-elision.html)
If it is unable to resolve the lifetimes by its usual rules, it will default to using the static lifetime.

By way of example:

```rust
// Resolved as `fn<'a>(&'a str) -> &'a str`.
const RESOLVED_SINGLE: fn(&str) -> &str = ..

// Resolved as `Fn<'a, 'b, 'c>(&'a Foo, &'b Bar, &'c Baz) -> usize`.
const RESOLVED_MULTIPLE: Fn(&Foo, &Bar, &Baz) -> usize = ..

// There is insufficient information to bound the return reference lifetime
// relative to the argument lifetimes, so the signature is resolved as
// `Fn(&'static Foo, &'static Bar) -> &'static Baz`.
const RESOLVED_STATIC: Fn(&Foo, &Bar) -> &Baz = ..
```
