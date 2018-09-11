# `Into` trait
https://doc.rust-lang.org/std/convert/trait.Into.html

Trait `std::convert::Into` 1.0.0

```rust
pub trait Into<T> {
    fn into(self) -> T;
}
```

A conversion that consumes `self`, which may or may not be expensive.
The reciprocal of `From`.
Note: this trait must not fail. If the conversion can fail, use `TryInto`
or a dedicated method which returns an `Option<T>` or a `Result<T, E>`.

Library authors should not directly implement this trait, but should prefer
implementing the `From` trait, which offers greater flexibility and provides an
equivalent `Into` implementation for free, thanks to a *blanket implementation*
in the standard library.


## Generic Implementations
`From<T> for U` implies `Into<U> for T`
`into` is reflexive, which means that `Into<T>` for `T` is implemented.


## Implementing Into
There is one exception to implementing `Into`, and it's kind of esoteric.
If the destination type is not part of the current crate, and it uses a generic
variable, then you can't implement `From` directly. For example, take this crate:

```rust
struct Wrapper<T>(Vec<T>);
impl<T> From<Wrapper<T>> for Vec<T> {
    fn from(w: Wrapper<T>) -> Vec<T> {
        w.0
    }
}
```
To fix this, you can implement `Into` directly:

```rust
struct Wrapper<T>(Vec<T>);
impl<T> Into<Vec<T>> for Wrapper<T> {
    fn into(self) -> Vec<T> {
        self.0
    }
}
```
This won't always allow the conversion: for example, `try!` and `?` always use 
`From`. However, in most cases, people use `Into` to do the conversions, and
this will allow that.

In almost all cases, you should try to implement `From`,
then fall back to `Into` if `From` can't be implemented.


## Examples
String implements `Into<Vec<u8>>`:

```rust
fn is_hello<T: Into<Vec<u8>>>(s: T) {
   let bytes = b"hello".to_vec();
   assert_eq!(bytes, s.into());
}

let s = "hello".to_string();
is_hello(s);
```

## Required Methods
```rust
fn into(self) -> T
```
Performs the conversion.


## Implementors
```rust
impl<T, U> Into<U> for T
  where U: From<T>
```
