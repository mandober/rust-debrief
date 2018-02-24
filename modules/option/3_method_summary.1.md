# Option methods by semantics 


## Check inner value
Make sure optional value is the expected one.

`Option<T>` => `bool`
- `is_some` returns true if `Some`, else false.
- `is_none` returns true if `None`, else false.


## Get inner value
The only way to get inner value from Option is to unwrap it, risking a panic if the value is absent.

`Some(v) `=> `v`
- `unwrap` moves value v out of `Some(v)` returning it, or panics.
- `expect` unwraps, or panics with a message.
- `unwrap_or` unwraps, or returns `param: T`.
- `unwrap_or_else` unwraps, or calls `FnOnce()->T`
- `unwrap_or_default` unwraps, or returns default for `T` type.



`Option<T>` => `Option<&T>`
- `as_ref` optional ref to inner value `Some(&T)`, or `None::<&T>`
- `as_mut` optional mut ref to inner value `Some(&mut T)`, or `None::<&mut T>`


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
- `*_or_else` may calculate return from supplied param-less closure
- `*_or_default` may return type's default value




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
