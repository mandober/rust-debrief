# Option methods

<!-- TOC -->

- [is_some](#is_some)
- [is_none](#is_none)
- [as_ref](#as_ref)
- [as_mut](#as_mut)
- [expect](#expect)
- [unwrap](#unwrap)
- [unwrap_or](#unwrap_or)
- [unwrap_or_else](#unwrap_or_else)
- [map](#map)
- [map_or](#map_or)
- [map_or_else](#map_or_else)
- [ok_or](#ok_or)
- [ok_or_else](#ok_or_else)
- [filter](#filter)
- [iter](#iter)
- [iter_mut](#iter_mut)
- [and](#and)
- [and_then](#and_then)
- [or](#or)
- [or_else](#or_else)
- [get_or_insert](#get_or_insert)
- [get_or_insert_with](#get_or_insert_with)
- [take](#take)
- [cloned](#cloned)
- [cloned](#cloned-1)
- [unwrap_or_default](#unwrap_or_default)
- [transpose](#transpose)

<!-- /TOC -->

---

`Option<T> `=> `T`
- `unwrap` moves `T` out of `Some(T)` returning it, or panics.
- `expect` unwraps, or panics with a message.
- `unwrap_or` unwraps, or returns `param: T`.
- `unwrap_or_else` unwraps, or calls `FnOnce()->T`
- `unwrap_or_default` unwraps, or returns default for `T` type.

`Option<T>` => `Option<&T>`
- `as_ref` returns `Some(&T)`, or `None::<&T>`.
- `as_mut` returns `Some(&mut T)`, or `None::<&mut T>`.


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


`Option<T>` => `bool`
- `is_some` returns true if `Some`, else false.
- `is_none` returns true if `None`, else false.

`Option<T>` => `Iter<T>`
- `iter` returns iterator over T, or empty iterator.
- `iter_mut` returns mut iterator over T, or empty iterator.

`Option<T>` => `Result<T, E>`
- `ok_or` transform `Option<T>` into `Result<T, E>`, E type of param.
- `ok_or_else` like ok_or, but E is computed from `FnOnce()->E`.

`Option<Result<T, E>>` => `Result<Option<T>, E>`
- `transpose` optional result into result of option


Conventions:
- `*_or` may return supplied parameter
- `*_or_else` may calculate return from supplied parameterless closure
- `*_or_default` may return type's default value



---

```rust
from Option<T> to:

T
U
Option<&T>, Option<&mut T>
Option<U>
Iter<T>, IterMut<T>
Result<T, E>
bool
```



Comparison
- map:      Some(in) => FnOnce(in)->out       => Some(out), None => None
- and_then: Some(in) => FnOnce(in)->Some(out) => Some(out), None => None
- or:       Some(in) => Some(in), None => FnOnce(in)->Some(in) => Some(in)


---

The following methods are implemented for:

```rust
impl<T> Option<T>
```

## is_some
- Option<T> => bool
- Returns `true` if `Some`, else `false`.
- Some(T) => true
- None => false

```rust
fn is_some(&self) -> bool;

let x: Option<u32> = Some(2);
let y: Option<u32> = None;
assert_eq!(x.is_some(), true);
assert_eq!(y.is_some(), false);
```


## is_none
- Option<T> => bool
- Returns `true` if `None`, else `false`.
- Some(T) => false
- None => true


```rust
fn is_none(&self) -> bool;

let y: Option<u32> = None;
assert_eq!(y.is_some(), false);
```


## as_ref
- Option<T> => Option<&T>
- Converts from `Option<T>` to `Option<&T>`.
- Weakens the inner type, T => &T
- if Some(T) => Some(&T)
- if None => None

```rust
fn as_ref(&self) -> Option<&T>;

// To convert Option<String> into Option<usize>, preserving the original:
// since map takes self by value, as_ref is used to make an Option containing
// a ref to the value inside.
let num_as_str: Option<String> = Some("10".to_string());
// First, cast Option<String> to Option<&String> with as_ref,
// then consume that with map, leaving num_as_str on the stack.
let num_as_int: Option<usize> = num_as_str.as_ref().map(|n| n.len());
println!("still can print num_as_str: {:?}", num_as_str);
```


## as_mut
- Option<T> => Option<&mut T>
- Converts from `Option<T>` to `Option<&mut T>`.
- Weakens the inner type, T => &mut T
- if Some(T) => Some(&mut T)
- if None => None

```rust
fn as_mut(&mut self) -> Option<&mut T>;

let mut x = Some(2);
match x.as_mut() {
    Some(v) => *v = 42,
    None => {},
}
assert_eq!(x, Some(42));
```


## expect
- Moves `T` out of `Option<T>` and returns it, or panics with a given message.
- if Some(T) => T
- if None => panics with the supplied message (of type `&str`).

```rust
fn expect(self, msg: &str) -> T;

let x = Some("value");
assert_eq!(x.expect("the world is ending"), "value");
let x: Option<&str> = None;
x.expect("the world is ending"); // panic
```


## unwrap
- Moves inner value out of `Option<T>` and returns it, or panics.
- if Some(T) => T
- if None => panics

```rust
fn unwrap(self) -> T;

let x = Some("air");
assert_eq!(x.unwrap(), "air");
let x: Option<&str> = None;
assert_eq!(x.unwrap(), "air"); // panic
```


## unwrap_or
- Moves `T` out of `Option<T>` and returns it
- Arguments passed to `unwrap_or` are eagerly evaluated; if you are passing the result of a function call, it is recommended to use `unwrap_or_else`, which is lazily evaluated.
- Moves and returns `T` out of `Option<T>`. If `None` returns supplied param.
- Some(v) => v::<T>
- None => param::<T>

```rust
fn unwrap_or(self, param: T) -> T;

assert_eq!(Some("car").unwrap_or("bike"), "car");
assert_eq!(None.unwrap_or("bike"), "bike");
```


## unwrap_or_else
- Returns the contained value or computes it from a closure.
- Arguments passed to `unwrap_or_else` are lazily evaluated.
- Some(T) => T   
  If `Some(T)`, moves and returns `T`
- None => FnOnce()->T   
  If `None`, calls the supplied parameter-less closure, `FnOnce()->T`, and returns its result (which must be of type `T`).


```rust
fn unwrap_or_else<F>(self, f: F) -> T 
   where F: FnOnce() -> T;

let k = 10;
assert_eq!(Some(4).unwrap_or_else(|| 2 * k), 4);
assert_eq!(None.unwrap_or_else(|| 2 * k), 20);
```


## map
Maps an `Option<T>` to `Option<U>` by applying a function to a contained value.
Returns `None` if the option is `None`.

Consumes `Option::Some(T)` by mapping it to `Option::Some(U)` using a closure. The closure receives the `T` as parameter, computes and returns the value, `U`. Then map returns this value, `U`, wrapped in an Option as `Option<U>`. If Option was `Option::None` to begin with, it is passed through.

- if Some:
  - consumes self i.e. the Option on which the map is called
  - applies supplied closure, `FnOnce(T) -> U`, to the inner value, possibly transforming the inner value's type into another.
  - returns result of closure, `U`, wrapping it in an option as `Option<U>`
  - Option::Some(T)::<T> => Option::Some(U)::<U>
- if None: map returns None as `Option::None::<U>`
  Option::None::<T> => Option::None::<U>


```rust
fn map<U, F>(self, f: F) -> Option<U>
   where F: FnOnce(T) -> U;

// map inner values
let opt_a: Option<String> = Some(String::from("Alpha"));
let opt_b: Option<String> = None;

// in case the Option is Some
let mapped: Option<usize> = opt_a.map(|s: String| s.len());
let expected = Option::Some::<usize>(5);
assert_eq!(mapped, expected);

// in case the Option is None
let mapped: Option<usize> = opt_b.map(|s: String| s.len());
assert_eq!(mapped, Option::None::<usize>);




let opt_b: Option<String> = Some(String::from("Alpha"));
let opt_a: Option<String> = None;
let closure = |s: String| s.len(); // s must be annotated in this context

// in case Option is None, it is just returned
let mapped = opt_a.map(closure);
assert_eq!(mapped, None);




// To convert `Option<String>` into `Option<usize>`, consuming the original
// maybe some string
let sng: Option<String> = Some(String::from("Alpha"));

// `Option::map` takes self by value, consuming `sng`
let len: Option<usize> = sng.map(|s: String| s.len());
// `sng` is no more

assert_eq!(len, Some(5));
```


## map_or
Applies a function to the contained value (if any), or returns a default.

Like `map`, but if contained value is `None`, supplied value is returned.

```rust
fn map_or<U, F>(self, default: U, f: F) -> U 
   where F: FnOnce(T) -> U;

let x = Some("foo");
assert_eq!(x.map_or(42, |v| v.len()), 3);
let x: Option<&str> = None;
assert_eq!(x.map_or(42, |v| v.len()), 42);
```


## map_or_else
Applies a fn to the contained value, or computes a default from another fn.

Like `map`, but if contained value is `None`, return value is computed from supplied closure.


```rust
fn map_or_else<U, D, F>(self, default: D, f: F) -> U
   where D: FnOnce() -> U,
         F: FnOnce(T) -> U;

let k = 21;
let x = Some("foo");
assert_eq!(x.map_or_else(|| 2 * k, |v| v.len()), 3);
let x: Option<&str> = None;
assert_eq!(x.map_or_else(|| 2 * k, |v| v.len()), 42);
```


## ok_or
Transforms the `Option<T>` into a `Result<T, E>`.

Transforms the `Option<T>` into a `Result<T, E>` by mapping `Some(v)` to 
`Ok(v)` and `None` to `Err(e)`, where `e` is the supplied value.

Arguments passed to `ok_or` are eagerly evaluated; if you are passing the result of a function call, it is recommended to use `ok_or_else`, which is lazily evaluated.

```rust
fn ok_or<E>(self, err: E) -> Result<T, E>;

let x = Some("foo");
assert_eq!(x.ok_or(0), Ok("foo"));

let x: Option<&str> = None;
assert_eq!(x.ok_or(0), Err(0));
```


## ok_or_else
Transforms the `Option<T>` into a `Result<T, E>`.

Transforms the `Option<T>` into a `Result<T, E>` by mapping `Some(v)` to 
`Ok(v)` and `None` to `Err(e)`, where `e` is computed from supplied closure (without input params).

Arguments passed to `ok_or_else`are lazily evaluated; recommended fn to use when passing the result of a function call (unlike eager arguments evaluation that `ok_or`does).

```rust
fn ok_or_else<E, F>(self, err: F) -> Result<T, E> 
   where F: FnOnce() -> E;

let x = Some("foo");
assert_eq!(x.ok_or_else(|| 0), Ok("foo"));
let x: Option<&str> = None;
assert_eq!(x.ok_or_else(|| 0), Err(0));
```

## filter
- This is a nightly-only experimental API. (option_filter [#45860](https://github.com/rust-lang/rust/issues/45860)
- Returns None if the option is None, otherwise calls predicate with the wrapped value and returns:
  - Some(t) if predicate returns true (where t is the wrapped value), and
  - None if predicate returns false.
This function works similar to `Iterator::filter()`. You can imagine the `Option<T>` being an iterator over one or zero elements. `filter()` lets you decide which elements to keep.

```rust
pub fn filter<P>(self, predicate: P) -> Option<T>
  where P: FnOnce(&T) -> bool;

#![feature(option_filter)]
fn is_even(n: &i32) -> bool { n % 2 == 0 }
assert_eq!(None.filter(is_even), None);
assert_eq!(Some(3).filter(is_even), None);
assert_eq!(Some(4).filter(is_even), Some(4));
```


## iter
Returns an iterator over the possibly contained value, or empty iterator if None.


```rust
fn iter(&self) -> Iter<T>;

let x = Some(4);
assert_eq!(x.iter().next(), Some(&4));

let x: Option<u32> = None;
assert_eq!(x.iter().next(), None);
```


## iter_mut
Returns a mutable iterator over the possibly contained value.

```rust
fn iter_mut(&mut self) -> IterMut<T>;

let mut x = Some(4);
match x.iter_mut().next() {
    Some(v) => *v = 42,
    None => {},
}
assert_eq!(x, Some(42));
let mut x: Option<u32> = None;
assert_eq!(x.iter_mut().next(), None);
```


## and
Returns `None` if the option is `None`, otherwise returns `optb`.

```rust
fn and<U>(self, optb: Option<U>) -> Option<U>;

let x = Some(2);
let y: Option<&str> = None;
assert_eq!(x.and(y), None);
```


## and_then
Returns `None` if the option is `None`, otherwise calls fn with the wrapped value and returns the result. Also known as **flatmap**.

```rust
fn and_then<U, F>(self, f: F) -> Option<U> 
   where F: FnOnce(T) -> Option<U>;

fn sq(x: u32) -> Option<u32> { Some(x * x) }
fn nope(_: u32) -> Option<u32> { None }
assert_eq!(Some(2).and_then(sq).and_then(sq), Some(16));
assert_eq!(Some(2).and_then(sq).and_then(nope), None);
assert_eq!(Some(2).and_then(nope).and_then(sq), None);
assert_eq!(None.and_then(sq).and_then(sq), None);
```


## or
Returns the option if it contains a value, otherwise returns supplied `optb`.

```rust
fn or(self, optb: Option<T>) -> Option<T>;

let x = Some(2);
let y = None;
assert_eq!(x.or(y), Some(2));
let x = None;
let y = Some(100);
assert_eq!(x.or(y), Some(100));
let x = Some(2);
let y = Some(100);
assert_eq!(x.or(y), Some(2));
let x: Option<u32> = None;
let y = None;
assert_eq!(x.or(y), None);
```


## or_else
Returns the option if it contains a value, otherwise calls f and returns the result.

```rust
fn or_else<F>(self, f: F) -> Option<T> where F: FnOnce() -> Option<T>;

fn nobody() -> Option<&'static str> { None }
fn vikings() -> Option<&'static str> { Some("vikings") }
assert_eq!(Some("barbarians").or_else(vikings), Some("barbarians"));
assert_eq!(None.or_else(vikings), Some("vikings"));
assert_eq!(None.or_else(nobody), None);
```


## get_or_insert
Inserts v into the option if it is None, then returns a mutable reference to the contained value. Since v.1.20.00

```rust
fn get_or_insert(&mut self, v: T) -> &mut T;

let mut x = None;
{   let y: &mut u32 = x.get_or_insert(5);
    assert_eq!(y, &5);
    *y = 7;
}
assert_eq!(x, Some(7));
```


## get_or_insert_with
Inserts a value computed from f into the option if it is None, then returns a mutable reference to the contained value.

```rust
fn get_or_insert_with<F>(&mut self, f: F) -> &mut T
  where F: FnOnce() -> T; //1.20

let mut x = None;
{   
  let y: &mut u32 = x.get_or_insert_with(|| 5);
  assert_eq!(y, &5);
  *y = 7;
}
assert_eq!(x, Some(7));
```


## take
Takes the Some variant out of the option, leaving a None variant in its place.

```rust
fn take(&mut self) -> Option<T>;

let mut x: Option<u32> = Some(2);
let z: Option<u32> = x.take();
assert_eq!(x, None);
assert_eq!(z, Some(2));
let mut x: Option<u32> = None;
x.take();
assert_eq!(x, None);
```



## cloned
Maps an `Option<&T>` to an `Option<T>` by cloning the contents of option.

```rust
impl<'a, T> Option<&'a T> where T: Clone {
  fn cloned(self) -> Option<T>;
  
  let x = 12;
  let opt_x = Some(&x);
  assert_eq!(opt_x, Some(&12));
  let cloned = opt_x.cloned();
  assert_eq!(cloned, Some(12));Run
}
```

## cloned

NIGHTLY: https://github.com/rust-lang/rust/issues/43738
Maps an `Option<&mut T>` to an `Option<T>` by cloning the contents of option.

```rust
impl<'a, T> Option<&'a mut T> where T: Clone {
  fn cloned(self) -> Option<T>;
  
  let x = 12;
  let opt_x = Some(&x);
  assert_eq!(opt_x, Some(&12));
  let cloned = opt_x.cloned();
  assert_eq!(cloned, Some(12));
}
```


## unwrap_or_default
Returns the contained value or a default

Consumes `self` then, if `Some`, returns the contained value, otherwise if `None`, returns the default value for that type. 

```rust
impl<'a, T> Option<&'a T> where T: Default {
  fn unwrap_or_default(self) -> T;
  
  // Convert a string to an integer, turning poorly-formed strings into 0
  // (the default value for integers). parse converts a string to any other
  // type that implements FromStr, returning None on error.
  let good_year_from_input = "1909";
  let bad_year_from_input = "190blarg";
  
  let good_year = good_year_from_input.parse().ok().unwrap_or_default();
  let bad_year = bad_year_from_input.parse().ok().unwrap_or_default();
  
  assert_eq!(1909, good_year);
  assert_eq!(0, bad_year);
}
```


## transpose

- Nightly-only experimental API   
  `transpose_result` [#47338](https://github.com/rust-lang/rust/issues/47338)
- Transposes an `Option` of a `Result` into a `Result` of an `Option`:
  - `None`         will be mapped to `Ok(None)`.
  - `Some(Ok(_))`  will be mapped to `Ok(Some(_))`.
  - `Some(Err(_))` will be mapped to `Err(_)`.

```rust
impl<T, E> Option<Result<T, E>> {
  pub fn transpose(self) -> Result<Option<T>, E>;

  #![feature(transpose_result)]
  #[derive(Debug, Eq, PartialEq)]
  struct SomeErr;
  let x: Result<Option<i32>, SomeErr> = Ok(Some(5));
  let y: Option<Result<i32, SomeErr>> = Some(Ok(5));
  assert_eq!(x, y.transpose());
}
```
