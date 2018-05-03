# Option

- Option represents optional value: either present or absent (alike `null`).
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

- [Option enum](#option-enum)
- [Option and null](#option-and-null)
- [Null-pointer optimization](#null-pointer-optimization)
- [Nullable pointers](#nullable-pointers)
- [Option methods by ownership](#option-methods-by-ownership)

<!-- /TOC -->



## Option enum

Option is not a special language item, if it weren't defined, it would be easy to implement it in safe Rust.

```rust
// Option definition in std::option::Option
pub enum Option<T> {
  Some(T),
  None,
}
```

Option enum is ideal for representing an optional value, a value that is either present or possibly absent. Option encodes this with two variants: variant `Some(T)` signifies presence of value `T` and variant `None` its absence. 

```rust
// None variants sometimes need annotatation
let x: Option<u32> = None;
let y: Option<&str> = None;
```

Option types are very common in Rust code, as they have a number of uses:
- Initial values
- Return values for functions that are not defined over their entire input range (partial functions)
- Stand-in for `Result`, replacing `Err` variant with `None`
- Optional `struct` fields; struct fields that can be loaned or "taken"
- Optional function arguments
- Swapping things out of difficult situations
- Nullable pointers:`Option<&T>` has the same memory representation as a nullable pointer, and can be passed across FFI boundaries as such.


## Option and null

Option is used whenever there is a chance that the value could be absent. It is similar to the concept of null (nil) from the other languages. In those languages, null is a separate type that has only one value `null` and null values are equal to each other. As such it is a super type, because it can stand in for any other type. On the other hand, in Rust, the None variant is always of the same type as the value whose absence it represents. Any time a `None` is seen, it is a `None` of some specific type, and because of this occasionally its type must be annotated:

```rust
// the conservative way
let opt: Option<u8> = None;

// turbofish
let opt = None::<u8>;

// swordfishtrombone
let opt: Option<u8> = Option::None::<u8>;
```


## Null-pointer optimization

Option is null-pointer optimized which means there is no memory overhead for putting any type into the Option; the size of `T` and the size of `Option<T>` is the same. 

As any enum, Option is as big as its biggest variant. 



## Nullable pointers

Option is suitable for representation of __nullable pointers__. `Option<&T>` has the same memory representation as a nullable pointer, it can be passed across FFI boundaries as such.


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
