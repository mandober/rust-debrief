# Clone trait
https://doc.rust-lang.org/std/clone/trait.Clone.html

Trait `std::clone::Clone` 1.0.0

A common trait for the ability to explicitly duplicate an object.

```rust
#[lang = "clone"]
pub trait Clone {
    fn clone(&self) -> Self;
    fn clone_from(&mut self, source: &Self) { ... }
}
```

Differs from Copy in that `Copy` is implicit and extremely inexpensive, while 
`Clone` is always explicit and may or may not be expensive. In order to enforce 
these characteristics, Rust does not allow you to reimplement Copy, but you may 
reimplement Clone and run arbitrary code.

Since Clone is more general than Copy, you can automatically make anything Copy 
be Clone as well.

This trait can be used with `#[derive]` if all fields are `Clone`. 
The derived implementation of clone calls clone on each field.


## impl Clone

Types that are `Copy` should have a trivial implementation of `Clone`.

More formally:
if `T: Copy`, `x: T`, and `y: &T`,
then `let x = y.clone()` is equivalent to `let x = *y`.

Manual implementations should be careful to uphold this invariant; 
however, unsafe code must not rely on it to ensure memory safety.

An example is an array holding more than 32 elements of a type that is `Clone`;
the standard library only implements `Clone` up until arrays of size 32. 
In this case, implementation of `Clone` cannot be derived, but can be impl as:

```rust
#[derive(Copy)]
struct Stats {
   frequencies: [i32; 100],
}

impl Clone for Stats {
    fn clone(&self) -> Stats { *self }
}
```


## Required Methods

```rust
fn clone(&self) -> Self
```
Returns a copy of the value.

Examples:
```rust
let hello = "Hello"; // &str implements Clone
assert_eq!("Hello", hello.clone());
```

## Provided Methods

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from source.

`a.clone_from(&b)` is equivalent to `a = b.clone()` in functionality, but can be 
overridden to reuse the resources of a to avoid unnecessary allocations.
