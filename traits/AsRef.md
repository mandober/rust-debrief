# Trait `std::convert::AsRef` 1.0.0
https://doc.rust-lang.org/std/convert/trait.AsRef.html

A cheap reference-to-reference conversion.
Used to convert a value to a reference value within generic code.

```rust
pub trait AsRef<T>
    where T: ?Sized {
    fn as_ref(&self) -> &T;
}
```

AsRef is very similar, but serves a slightly different purpose than, Borrow.
`AsRef` is to be used when wishing to *convert to a reference* of another type.
`Borrow` is more related to the notion of *taking the reference*.

It is useful when wishing to abstract over the type of reference (`&T`, `&mut T`) 
or allow both the referenced and owned type to be treated in the same manner.

The key difference between the two traits is the intention:
- Use `AsRef` when goal is to simply *convert into a reference*.
- Use `Borrow` when goal is related to writing code that is
  *agnostic to the type of borrow* and *wheter it is a reference or a value*.


**This trait must not fail**
If the conversion can fail, use a dedicated method
which returns an `Option<T>` or a `Result<T, E>`.



## Generic Implementations

`AsRef` auto-dereferences if the inner type is a reference or a mutable reference 
(e.g: `foo.as_ref()` will work the same if `foo` has type `&mut Foo` or `&&mut Foo`).

Examples:

Both `String` and `&str` implement `AsRef<str>`:

```rust
fn is_hello<T: AsRef<str>>(s: T) {
   assert_eq!("hello", s.as_ref());
}

let s = "hello";
is_hello(s);

let s = "hello".to_string();
is_hello(s);
```


## Required Methods

```rust
fn as_ref(&self) -> &T
```
Performs the conversion.