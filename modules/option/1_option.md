# Option


## Debrief
- Option represents optional value: either present or absent (akin to `null`).
- Option enum lives in`std::option` module
- Online docs: [std::option][mods], [std::option::Option][enum]
- This module also contains structs, used for iteration over Option:
  - `IntoIter` iterator over the value in Some
  - `Iter` iterator over a ref to the Some
  - `IterMut` iterator over a mut ref to the Some
  - These iterators produce one value (Some) at most.
- `Option<T>` is a generic type, a wrapper over some type `T`
- `Option<T>` has 2 variants:
  - `Some(T)` if the value of type `T` is present, `Option::Some::<T>(T)`
  - `None` if the value, __of that same type `T`__, is absent.
  - This means every optional value has its own None: `Option::None::<T>`
- Option is null-pointer optimized
  - the size of `T` and `Option<T>` is the same.
  - `Option<&T>` has the same memory representation as a nullable pointer, and can be passed across FFI boundaries as such.


[mods]: https://doc.rust-lang.org/nightly/std/option/
[enum]: https://doc.rust-lang.org/nightly/std/option/enum.Option.html


<!-- TOC -->

- [Debrief](#debrief)
- [Option enum](#option-enum)
- [Option and NULL](#option-and-null)
- [Optional values](#optional-values)
- [Nullable pointers](#nullable-pointers)
- [Methods by purpose](#methods-by-purpose)
- [Option methods by ownership](#option-methods-by-ownership)

<!-- /TOC -->


## Option enum

Option is not a special language item, if it didn't exist it would be easy to implement it in safe Rust.

```rust
// Option definition in std::option::Option
pub enum Option<T> {
    Some(T),
    None
}
```

Both of Option's variants are tuples: `Some(T)` is a named tuple with one element of type `T`. `None` variant is an empty tuple, `None()`.

Option represents an optional value; if it is possible that the value of some type could be missing or not be known, then Option is used to encode such situation. It is similar to the concept of `null` from other languages; Even though in such languages `null` represents the absence of some particular value, it also represents the absence of __any__ value, meaning that `null` is the separate type from the type whose absence it represents; therein null is equal to null. In Rust however, each optional type has its own `None` variant; any time a `None` is seen in the code, it is a `None` of some particular type, and because of this, occasionally its type must be annotated:

```rust
// the conservative way
let opt: Option<u8> = None;

// turbofish
let opt = None::<u8>;

// swordfishtrombone
let opt: Option<u8> = Option::None::<u8>;
```

Option is mostly used to represent optional values, but also initial values, return values for partial functions, as a stand-in for simple errors, optional struct fields, optional function args, nullable pointers, etc.

Option is null-pointer optimized which means there is no memory overhead for putting any type into the Option; the size of `T` and the size of `Option<T>` is the same.

Option is suitable for representation of nullable pointers; `Option<&T>` has the same memory representation as a nullable pointer, it can be passed across FFI boundaries as such.



## Option and NULL

Option enum is ideal for representing an optional value, a value that is either present or possibly absent; as such it is similar to a `null` value in other languages. Option encodes this with two variants: variant `Some(T)` signifies presence of value `T` and variant `None` its absence. 


```rust
// None variants sometimes need annotatation
let x: Option<u32> = None;
let y: Option<&str> = None;
```

## Optional values

`Option` types are very common in Rust code, as they have a number of uses:
- Initial values
- Return values for functions that are not defined over their entire input range (partial functions)
- Stand-in for `Result`, replacing `Err` variant with `None`
- Optional `struct` fields; struct fields that can be loaned or "taken"
- Optional function arguments
- Swapping things out of difficult situations
- Nullable pointers:`Option<&T>` has the same memory representation as a nullable pointer, and can be passed across FFI boundaries as such.



## Nullable pointers

Rust's pointer types must always point to a valid location, there are no null pointers. Instead, Rust has optional pointers, like the optionally owned box, `Option<Box<T>>`

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

This usage of `Option` to create safe nullable pointers is so common that Rust does special optimizations to make the representation of `Option<Box<T>>` a single pointer. Optional pointers in Rust are stored as efficiently as any other pointer type.






## Option methods by ownership

Unless otherwise noted, methods are for `impl<T> Option<T>`

```rust
// self by ref
fn is_some(&self) -> bool;
fn is_none(&self) -> bool;
fn as_ref(&self) -> Option<&T>;
fn iter(&self) -> Iter<T>;

// self by mut ref
fn get_or_insert(&mut self, v: T) -> &mut T;
fn get_or_insert_with<F>(&mut self, f: F) -> &mut T  where F: FnOnce()->T;
fn take(&mut self) -> Option<T>;
fn iter_mut(&mut self) -> IterMut<T>;
fn as_mut(&mut self) -> Option<&mut T>;

// self by value
fn expect(self, msg: &str) -> T;
fn unwrap(self) -> T;
fn unwrap_or(self, def: T) -> T;
fn unwrap_or_else<F>(self, f: F) -> T;
fn unwrap_or_default(self) -> T; // impl<T: Default> Option<T>
fn cloned(self) -> Option<T>;    // impl<'a, T: Clone> Option<&'a T>
fn cloned(self) -> Option<T>;    // impl<'a, T: Clone> Option<&'a mut T>
fn map<U, F>(self, f: F) -> Option<U>;
fn map_or<U, F>(self, default: U, f: F) -> U;
fn map_or_else<U, D: FnOnce()->U, F: FnOnce(T)->U>(self, gx: D, fx: F) -> U;
fn ok_or<E>(self, err: E) -> Result<T, E>;
fn ok_or_else<E, F>(self, err: F) -> Result<T, E>;
fn and<U>(self, optb: Option<U>) -> Option<U>;
fn and_then<U, F>(self, f: F) -> Option<U>  where F: FnOnce(T)->Option<U>;
fn filter<P>(self, predicate: P) -> Option<T>  where P: FnOnce(&T)->bool;
fn or(self, optb: Option<T>) -> Option<T>;
fn or_else<F>(self, f: F) -> Option<T>  where F: FnOnce()->Option<T>;
```
