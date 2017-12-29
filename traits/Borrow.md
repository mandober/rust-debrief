# Borrow trait
https://doc.rust-lang.org/std/borrow/trait.Borrow.html

Trait `std::borrow::Borrow` 1.0.0

A trait for borrowing data.

```rust
pub trait Borrow<Borrowed>
    where Borrowed: ?Sized {
    fn borrow(&self) -> &Borrowed;
}
```

In general, there may be several ways to "borrow" a piece of data.
The typical ways of borrowing a type `T` are `&T` (a shared borrow) and 
`&mut T` (a mutable borrow). But types like `Vec<T>` provide additional
kinds of borrows: the (mutably) borrowed slices `&[T]` and `&mut [T]`.

When writing generic code, it is often desirable to abstract over all ways of
borrowing data from a given type. That is the role of the `Borrow trait`:
if `T: Borrow<U>`, then `&U` can be borrowed from `&T`.
A given type can be borrowed as multiple different types.
In particular, `Vec<T>: Borrow<Vec<T>>` and `Vec<T>: Borrow<[T]>`.

If you are implementing `Borrow` and both `Self` and `Borrowed` 
implement `Hash`, `Eq`, and/or `Ord`, they must produce the same result.

`Borrow` is very similar to, but different than, `AsRef`.


## Required Methods

```rust
fn borrow(&self) -> &Borrowed
```
Immutably borrows from an owned value.


Examples:

```rust
use std::borrow::Borrow;

fn check<T: Borrow<str>>(s: T) {
    assert_eq!("Hello", s.borrow());
}

let s = "Hello".to_string();
check(s);
let s = "Hello";
check(s);
```
