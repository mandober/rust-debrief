# Methods

- Strictly, _methods_ are functions implemented for a type that have `self` as a parameter; those that don't have self as parameter are _associated functions_.
- The parameter `self` refers to an instance of the type, it comes in 3 forms, which are just shorthands:
  - `self` is shorthand for `self: Self`
  - `&self` is shorthand for `self: &Self`
  - `&mut self` shorthand for `self: &mut Self`
- The type `Self` is the signature of the type that the method is being `impl`emented for; it is often seen as method's return type.
- Methods can:
  - take ownership of self, consume self: `self`
  - borrow self immutably, read self: `&self`
  - borrow self mutably, write (to) self, mutate it: `&mut self`
- dot operator performs auto-(de)referencing of its operator: it adds as many 
  `&`, `&mut` or `*` as needed until an instance matches the method's signature.



```rust
// a type
struct<T> Driver<'a, T> {
  car: Option<&'a T>
}

impl<'a, T> Driver<'a, T> {
  // associated function
  fn new() -> Self {
    // Self is shorthand for: Driver<'a, T>
  }

  fn write(&mut self) -> Self {
    // &mut self is shorthand for:
    // self: &mut Self
  }
}
```
