# Option


## Option module
- Option module, `std::option`, since 1.0.0, [doc](https://doc.rust-lang.org/stable/std/option/index.html)
- Enums in Option module: `Option` enum
- Structs in Option module:
  - `IntoIter`, iterator over the value in `Some` variant.
  - `Iter`, iterator over a reference to the `Some` variant.
  - `IterMut`, an iterator over a mutable ref to the `Some` variant.


## `Option` enum
- Enums in Option module: `Option` enum, `std::option::Option`, since 1.0.0, [doc](https://doc.rust-lang.org/stable/std/option/enum.Option.html)
- Option is suitable for encoding an optional value: a value that is either present or absent; as such it is similar to the concept of `null` in other languages. Variant `Some(T)` signifies presence of value `T`, variant `None` its absence. However, this means there are multitude of `None` variants.
- Option is null-pointer optimized which means there is no memory overhead for putting a type into an Option; the size of `T` and the size of `Option<T>` is the same.
- Suitable for representation of nullable pointers; `Option<&T>` has the same memory representation as a nullable pointer, it can be passed across FFI boundaries as such.


```rust
// Option enum definition in std::option::Option:
pub enum Option<T> {
    Some(T),
    None
}
```


### Option enum and `null`

Option enum is ideal for representing an optional value, a value that is either present or possibly absent; as such it is similar to a `null` value in other languages. Option encodes this with two variants: variant `Some(T)` signifies presence of value `T` and variant `None` its absence. However, this means there is a multitude of `None` variants, which are not mutualy equal; each Option type has its own `None` variant (of that type). Both of this variants are tuples; `Some(T)` holds a tuple with a single value and `None()` has an empty tuple, which is left out as is commonly the case when it is the only or the last value.

```rust
// None variants sometimes need annotatation
let x: Option<u32> = None;
let y: Option<&str> = None;
```



### Option enum: method index (by semantics)

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


### Option enum: methods with examples

```rust
// is_some
fn is_some(&self) -> bool;
// returns true if the option is present.
let x: Option<u32> = Some(2);
assert_eq!(x.is_some(), true);

// is_none
fn is_none(&self) -> bool;
// returns true if the option is absent.
assert_eq!(x.is_none(), false);
```



### Option enum: method index (by ownership)

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
