# Option
https://doc.rust-lang.org/stable/std/option/index.html
Module `std::option` 1.0.0

## Optional values
Type `Option` represents an optional value: every `Option` is either `Some` and 
contains a value, or `None`, and does not. 

`Option` types are very common in Rust code, as they have a number of uses:
- Initial values
- Return values for functions that are not defined
  over their entire input range (partial functions)
- Return value for otherwise reporting simple 
  errors, where `None` is returned on error
- Optional `struct` fields
- `struct` fields that can be loaned or "taken"
- Optional function arguments
- Swapping things out of difficult situations
- Nullable pointers
  `Option<&T>` has the same memory representation as a nullable pointer, 
  and can be passed across FFI boundaries as such.



## Options and pointers ("nullable" pointers)

Rust's *pointer* types must always point to a valid location; *no null pointers*. 
Instead, Rust has *optional pointers*, 
like the optional owned box, `Option<Box<T>>`

The following example uses `Option` to create an optional box of i32. Notice 
that in order to use the inner i32 value first, the check_optional function 
needs to use pattern matching to determine whether the box has a value.

```rust
let optional = None;
check_optional(optional);

let optional = Some(Box::new(9000));
check_optional(optional);

fn check_optional(optional: Option<Box<i32>>) {
    match optional {
        Some(ref p) => println!("has value {}", p),
        None => println!("has no value"),
    }
}
```

This usage of `Option` to create safe nullable pointers is so common that Rust 
does special optimizations to make the representation of `Option<Box<T>>` a 
single pointer. Optional pointers in Rust are stored as efficiently as any other 
pointer type.


## Structs

`IntoIter` An iterator over the *value* in `Some` variant of an `Option`.
`Iter`     An iterator over a *reference* to the `Some` variant of an `Option`.
`IterMut`  An iterator over a *mutable reference* to the `Some` variant of an `Option`.

## Enums

`Option` The Option type. See the module level documentation for more.
