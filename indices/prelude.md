# The Rust Prelude
https://doc.rust-lang.org/nightly/std/prelude/

Module `std::prelude` v1.0.0

## Prelude contents
Prelude v1.0.0 lives in `std::prelude::v1` and re-exports:

1. `std::marker::{Copy, Send, Sized, Sync}`
    marker traits indicate fundamental properties of types

2. `std::ops::{Drop, Fn, FnMut, FnOnce}`
    various operations for both destructors and overloading `()`.

3. `std::mem::drop`
    convenience function for explicitly dropping a value.

4. `std::boxed::Box`
    a way to allocate values on the heap.

5. `std::borrow::ToOwned`
    The conversion trait that defines `to_owned`, the generic 
    method for creating an owned type from a borrowed type.

6. `std::clone::Clone` the ubiquitous trait that defines 
    `clone`, the method for producing a copy of a value.

7. `std::cmp::{PartialEq, PartialOrd, Eq, Ord }` 
    Comparison traits, implements the comparison 
    operators; often seen in trait bounds.

8. `std::convert::{AsRef, AsMut, Into, From}`
    Generic conversions to create overloaded methods.

9. `std::default::Default`
    types that have default values.

10. `std::iter::{Iterator, Extend, IntoIterator,`
    `DoubleEndedIterator, ExactSizeIterator}`
     Iterators of various kinds.

11. `std::option::Option::{self, Some, None}`
     type which expresses the presence or absence 
     of a value; variants are also exported.

12. `std::result::Result::{self, Ok, Err}`
     provides a few useful methods on slices.

13. `std::string::{String, ToString}`
     heap allocated strings.

14. `std::vec::Vec`
     a growable, heap-allocated vector.


The prelude is automatically imported into every Rust program, it's like
`extern crate std` is inserted into the crate root of every crate, and 
`use std::prelude::v1::*` into every module.
