# Primitive: string slice
https://doc.rust-lang.org/std/primitive.str.html

Primitive Type `str` 1.0.0

The `str` type, a *string slice*, is the most primitive string type.
It is usually seen in its borrowed form, `&str`.
It is also the type of *string literals*, `&'static str`.
String literals are string slices.

Strings slices are always valid UTF-8.

Strings slices have fixed size, they are a view into String.

This documentation describes a number of methods 
and trait implementations on the `str` type. 
For technical reasons, there is additional, separate 
documentation in the `std::str` module as well.

## Examples

```rust
// String literals are string slices:
let hello = "Hello, world!";
// with an explicit type annotation
let hello: &'static str = "Hello, world!";
```

They are `'static` because they're stored directly in the final binary, 
and so will be valid for the `'static` duration.


## Representation

A `&str` is made up of two components: 
- a pointer (on stack) to some bytes (on heap)
- a length

You can look at these with the `as_ptr` and `len` methods:

```rust
use std::slice;
use std::str;

let story = "Once upon a time...";

let ptr = story.as_ptr();
let len = story.len();

// story has nineteen bytes
assert_eq!(19, len);

// We can re-build a str out of ptr and len. This is all unsafe because
// we are responsible for making sure the two components are valid:
let s = unsafe {
    // First, we build a &[u8]...
    let slice = slice::from_raw_parts(ptr, len);

    // ... and then convert that slice into a string slice
    str::from_utf8(slice)
};

assert_eq!(s, Ok(story));
```
Note:
This example shows the internals of `&str`. 
`unsafe` should not be used to get a string slice under normal circumstances.
Use `as_slice` instead.