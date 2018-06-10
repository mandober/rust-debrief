# Option methods by semantics 

<!-- TOC -->

- [Identify](#identify)
- [Get inner value](#get-inner-value)
- [Get optional ref to inner value](#get-optional-ref-to-inner-value)
- [Mapping](#mapping)
- [Iteration](#iteration)
- [Result](#result)
- [Methods by purpose](#methods-by-purpose)

<!-- /TOC -->


Conventions:
- `*_or` caller supplies value that may be returned
- `*_or_else` caller supplies closure that may be used to calculate return val
- `*_or_default` may return type's default value



## Identify
Identify inner value (before unwrapping).

`Option<T>` => `bool`
- `is_some` returns true if `Some`, false if `None`.
- `is_none` returns true if `None`, false if `Some`.


## Get inner value
The only way to get inner value from Option is to unwrap it (risking a panic)

`Some(v) `=> `v`
- `unwrap` moves value v out of `Some(v)` returning it, or panics.
- `expect` unwraps, or panics with a message.
- `unwrap_or` unwraps, or returns supplied parameter of type `T`.
- `unwrap_or_else` unwraps, or calls `FnOnce()->T`
- `unwrap_or_default` unwraps, or returns default for `T` type.


## Get optional ref to inner value
and feed it to the methods that consume the Option (e.g. map) to preserve original Option. Type weakening.

`Option<T> => Option<&T>`
- `as_ref`: optional ref to inner value, `Some::<&T>(&v)` or `None::<&T>`
- `as_mut`: optional mut ref, `Some::<&mut T>(&mut v)`, or `None::<&mut T>`


## Mapping

`Option<T>` => `Option<U>`
- `map` maps `Option<T>` to `Option<U>` via fn `FnOnce(T)->U`
- `and` returns supplied param of type `Option<U>`, or None::<U>
- `and_then` returns result of `FnOnce(T)->Option<U>` on `T`, or None.

`Option<T>` => `U`
- `map_or` maps `T` with `FnOnce(T)->U`, or returns supplied param.
- `map_or_else` maps, or returns output of `FnOnce()->U`.

`Option<T>` => `Option<T>`
- `or` returns `Some(T)`, or param of type `Option<T>`.
- `or_else` returns `Some(T)`, or result of fn `FnOnce()->Option<T>`
- `filter` calls `FnOnce(&T)->bool` on `T` and returns `Some(T)` if true.

## Iteration

`Option<T>` => `Iter<T>`
- `iter` returns iterator over T, or empty iterator.
- `iter_mut` returns mut iterator over T, or empty iterator.


## Result

`Option<T>` => `Result<T, E>`
- `ok_or` transform `Option<T>` into `Result<T, E>`, E type of param.
- `ok_or_else` like ok_or, but E is computed from `FnOnce()->E`.

`Option<Result<T, E>>` => `Result<Option<T>, E>`
- `transpose` optional result into result of option




```rust
from Option<T> to:

bool
T
U
Option<&T>, Option<&mut T>
Option<U>
Iter<T>, IterMut<T>
Result<T, E>
```



Comparison
- map:      Some(in) => FnOnce(in)->out       => Some(out), None => None
- and_then: Some(in) => FnOnce(in)->Some(out) => Some(out), None => None
- or:       Some(in) => Some(in), None => FnOnce(in)->Some(in) => Some(in)


---


## Methods by purpose

Unless otherwise noted, methods are on `impl<T> Option<T>`

```rust

impl<T> Option<T> {
  
}


// get variant. identify inner value
fn is_some(&self) -> bool;
fn is_none(&self) -> bool;


// get opt ref to inner. Option<T> => Option<&T>
fn as_ref(&self) -> Option<&T>;
fn as_mut(&mut self) -> Option<&mut T>;

// get ref mut to inner (or first insert if None). Option<T> ==> &mut T
fn get_or_insert(&mut self, v: T) -> &mut T;
fn get_or_insert_with<F: FnOnce()->T>(&mut self, f: F) -> &mut T;

// get T by unwrapping inner
fn expect(self, msg: &str) -> T;
fn unwrap(self) -> T;
fn unwrap_or(self, def: T) -> T;
fn unwrap_or_else<F>(self, f: F) -> T;
fn unwrap_or_default(self) -> T; // impl<T: Default> Option<T>


// get opt T by replacing (stealing) inner. orig Option<T> => new Option<T>
fn take(&mut self) -> Option<T>;


// get another by cloning. Option<&'a T> ==> Option<T>
impl<'a, T: Clone> Option<&'a T> {
  fn cloned(self) -> Option<T>;
}

// get another by cloning. Option<&'a mut T> ==> Option<T>
impl<'a, T: Clone> Option<&'a mut T> {
  fn cloned(self) -> Option<T>;
}


// iterate (on Some)
fn iter(&self) -> Iter<T>;
fn iter_mut(&mut self) -> IterMut<T>;



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
```

