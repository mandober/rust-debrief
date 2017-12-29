# `AsMut` trait
https://doc.rust-lang.org/std/convert/trait.AsMut.html

Trait `std::convert::AsMut` 1.0.0

A cheap, mutable reference-to-mutable reference conversion.

```rust
pub trait AsMut<T> 
    where T: ?Sized {
    fn as_mut(&mut self) -> &mut T;
}
```

This trait is similar to `AsRef` but used
for converting between mutable references.

Note: __this trait must not fail__.
If the conversion can fail, use a dedicated method 
which returns an `Option<T>` or a `Result<T, E>`.


## Generic Implementations

`AsMut` auto-dereferences if the inner type is a reference or a mutable reference 
(e.g.: foo.as_ref() will work the same if foo has type &mut Foo or &&mut Foo).

Examples:

`Box<T>` implements `AsMut<T>`:

```rust
fn add_one<T: AsMut<u64>>(num: &mut T) {
    *num.as_mut() += 1;
}
let mut boxed_num = Box::new(0);
add_one(&mut boxed_num);
assert_eq!(*boxed_num, 1);
```

## Required Methods

```rust
fn as_mut(&mut self) -> &mut T
```

Performs the conversion.