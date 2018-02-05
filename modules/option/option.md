# Option type

- `std::option::Option` enum, since 1.0.0
- online: [docs](https://doc.rust-lang.org/stable/std/option/enum.Option.html)
- stands for optional value: `Option<T>`is either `Some(T)` or `None`
- null-pointer optimized, suitable for representation of nullable pointers



<!-- TOC -->

- [Option enum](#option-enum)
- [Option and NULL](#option-and-null)
- [Optional values](#optional-values)
- [Options and pointers ("nullable" pointers)](#options-and-pointers-nullable-pointers)
- [Option methods by semantics](#option-methods-by-semantics)
- [Option methods by ownership](#option-methods-by-ownership)

<!-- /TOC -->


## Option enum

Option enum represents an optional value: `Option<T>`is either `Some(T)` or `None: `a value that is either present or absent; as such it is similar to the concept of `null` in other languages.

It has 2 variants: variant `Some(T)` signifies presence of value `T`, variant `None` its absence.

```rust
// Option enum definition in std::option::Option:
pub enum Option<T> {
    Some(T),
    None
}
```

mostly used to represent optional values, but also initial values, return values for partial functions, as a stand-in for simple errors, optional struct fields, optional function args, nullable pointers, etc.

Option is null-pointer optimized enum which means there is no memory overhead for putting any type into the Option; the size of `T` and the size of `Option<T>` is the same.

suitable for representation of nullable pointers; `Option<&T>` has the same memory representation as a nullable pointer, it can be passed across FFI boundaries as such.


## Option and NULL

Option enum is ideal for representing an optional value, a value that is either present or possibly absent; as such it is similar to a `null` value in other languages. Option encodes this with two variants: variant `Some(T)` signifies presence of value `T` and variant `None` its absence. However, this means there is a multitude of `None` variants, which are not mutually equal; each Option type has its own `None` variant (of that type). Both of this variants are tuples; `Some(T)` holds a tuple with a single value and `None()` has an empty tuple, which is left out as is commonly the case when it is the only or the last value.

```rust
// None variants sometimes need annotatation
let x: Option<u32> = None;
let y: Option<&str> = None;
```

## Optional values
Type `Option` represents an optional value: every `Option` is either `Some` and 
contains a value, or `None`, and does not. 

`Option` types are very common in Rust code, as they have a number of uses:
- Initial values
- Return values for functions that are not defined over their entire input range (partial functions)
- Stand-in for `Result`, replacing `Err` variant with `None`
- Optional `struct` fields; struct fields that can be loaned or "taken"
- Optional function arguments
- Swapping things out of difficult situations
- Nullable pointers:`Option<&T>` has the same memory representation as a nullable pointer, and can be passed across FFI boundaries as such.



## Options and pointers ("nullable" pointers)

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





## Option methods by semantics

Unless otherwise noted, methods are for `impl<T> Option<T>`

```rust
// identify
fn is_some(&self) -> bool;
fn is_none(&self) -> bool;

// weaken T: Option<T> => Option<&T>
fn as_ref(&self) -> Option<&T>;
fn as_mut(&mut self) -> Option<&mut T>;

// unwrap. get T: Option<T> => T
fn expect(self, msg: &str) -> T;
fn unwrap(self) -> T;
fn unwrap_or(self, def: T) -> T;
fn unwrap_or_else<F>(self, f: F) -> T;
fn unwrap_or_default(self) -> T; // impl<T: Default> Option<T>

// replace: Some(T) => None, return Option<T>
fn take(&mut self) -> Option<T>;

// convert: Option<T> => Option<U> by applying fn to the wrapped value.
// where T is T, &T, &mut T
fn map<U, F>(self, f: F) -> Option<U>;
fn map_or<U, F>(self, default: U, f: F) -> U;
fn map_or_else<U, D, F>(self, default: D, f: F) -> U
  where D: FnOnce() -> U, F: FnOnce(T) -> U;

// transform: Option<T> => Result<T, E>
fn ok_or<E>(self, err: E) -> Result<T, E>;
fn ok_or_else<E, F>(self, err: F) -> Result<T, E>;

// alternate: return Option<U> if Option<T>::None
fn and<U>(self, optb: Option<U>) -> Option<U>;
fn and_then<U, F>(self, f: F) -> Option<U>  where F: FnOnce(T)->Option<U>;

fn filter<P>(self, predicate: P) -> Option<T>  where P: FnOnce(&T)->bool;

fn or(self, optb: Option<T>) -> Option<T>;
fn or_else<F>(self, f: F) -> Option<T>  where F: FnOnce()->Option<T>;

fn get_or_insert(&mut self, v: T) -> &mut T;
fn get_or_insert_with<F>(&mut self, f: F) -> &mut T  where F: FnOnce()->T;

// iterate
fn iter(&self) -> Iter<T>;
fn iter_mut(&mut self) -> IterMut<T>;

// clone: Option<T> => Option<T>
fn cloned(self) -> Option<T>; // impl<'a, T: Clone> Option<&'a T>
fn cloned(self) -> Option<T>; // impl<'a, T: Clone> Option<&'a mut T>
```


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
fn map_or_else<U, D, F>(self, default: D, f: F) -> U
  where D: FnOnce() -> U, F: FnOnce(T) -> U;
fn ok_or<E>(self, err: E) -> Result<T, E>;
fn ok_or_else<E, F>(self, err: F) -> Result<T, E>;
fn and<U>(self, optb: Option<U>) -> Option<U>;
fn and_then<U, F>(self, f: F) -> Option<U>  where F: FnOnce(T)->Option<U>;
fn filter<P>(self, predicate: P) -> Option<T>  where P: FnOnce(&T)->bool;
fn or(self, optb: Option<T>) -> Option<T>;
fn or_else<F>(self, f: F) -> Option<T>  where F: FnOnce()->Option<T>;
```
