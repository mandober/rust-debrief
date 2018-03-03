# `Copy` trait
https://doc.rust-lang.org/std/marker/trait.Copy.html

Trait `cmp::marker::Copy` 1.0.0

Types whose values can be duplicated simply by copying bits.

```rust
#[lang = "copy"]
pub trait Copy: Clone { }
```

## Summary
- `Copy` types: types whose values can be duplicated simply by copying bits.
  By default, variable bindings have 'move semantics'.
  However, if a type implements `Copy`, it instead has 'copy semantics'
- Impl `Copy` by deriving (via attr) or by manual impl.
  deriving will also place a `Copy` bound on type param.
- Copies happen implicitly. 
- `Copy` is not overloadable
- `Clone` is a supertrait of `Copy`, so all `Copy` types must also impl `Clone`. 
  If a type is `Copy` then its `Clone` impl only needs to return `*self`.
- A type can implement `Copy` if all of its components implement `Copy`. 
- any type implementing `Drop` can't be `Copy`, because it's managing some 
  resource besides its own `size_of::<T>` bytes.






## Characteristics

By default, variable bindings have 'move semantics.' In other words:

```rust
#[derive(Debug)]
struct Foo;
let x = Foo;
let y = x;
// `x` has moved into `y`, and so cannot be used
// println!("{:?}", x); // error: use of moved value
```

However, if a type implements Copy, it instead has 'copy semantics':

```rust
// We can derive a `Copy` implementation.
// `Clone` is also required, as it's a supertrait of `Copy`.
#[derive(Debug, Copy, Clone)]
struct Foo;
let x = Foo;
let y = x;
// `y` is a copy of `x`
println!("{:?}", x); // A-OK!
```

It's important to note that in these two examples, the only difference is whether 
you are allowed to access x after the assignment. Under the hood, both a copy 
and a move can result in bits being copied in memory, although this is sometimes 
optimized away.


## Implementing `Copy`

There are two ways to implement `Copy` on your type. 
The simplest is to use derive:

```rust
#[derive(Copy, Clone)]
struct MyStruct;
```

You can also implement `Copy` and `Clone` manually:

```rust
struct MyStruct;

impl Copy for MyStruct { }

impl Clone for MyStruct {
    fn clone(&self) -> MyStruct {
        *self
    }
}
```
There is a small difference between the two: the derive strategy will 
also place a `Copy` bound on type parameters, which isn't always desired.


## Difference between `Copy` and `Clone`

Copies happen implicitly, for example as part of an assignment `y = x`. 
The behavior of `Copy` is not overloadable; it is always a simple bit-wise copy.

Cloning is an explicit action, `x.clone()`. The implementation of `Clone` can 
provide any type-specific behavior necessary to duplicate values safely.

For example, the implementation of `Clone` for `String` needs to copy the 
pointed-to string buffer in the heap. A simple bitwise copy of `String` values 
would merely copy the pointer, leading to a double free down the line. 
For this reason, `String` is `Clone` but not `Copy`.

`Clone` is a supertrait of `Copy`, so everything which is `Copy` must also 
implement `Clone`. If a type is `Copy` then its `Clone` implementation only 
needs to return `*self` (see the example above).



## When can my type be `Copy`?
A type can implement `Copy` if all of its components implement `Copy`. 
For example, this struct can be `Copy`:

```rust
struct Point {
   x: i32,
   y: i32,
}
```
A struct can be `Copy`, and `i32` is `Copy`, therefore `Point` is eligible to be 
`Copy`. By contrast, consider:

```rust
struct PointList {
    points: Vec<Point>,
}
```
The struct `PointList` cannot implement `Copy`, because `Vec<T>` is not `Copy`. 
If we attempt to derive a `Copy` implementation, we'll get an error:
  "the trait Copy may not be implemented for this type;
  field `points` does not implement Copy".


## When can't my type be Copy?
Some types can't be copied safely. 
For example, copying mut ref (`&mut T`) would create an aliased mutable ref. 
Copying `String` would duplicate responsibility for managing the String's buffer, 
leading to a double free.

Generalizing the latter case, any type implementing `Drop` can't be `Copy`, 
because it's managing some resource besides its own `size_of::<T>` bytes.

If you try to implement Copy on a struct or enum containing non-Copy data, you 
will get the error *E0204*.


## When should my type be Copy?
Generally speaking, if your type can implement `Copy`, it should. Keep in mind, 
though, that implementing `Copy` is part of the public API of your type. If the 
type might become non-Copy in the future, it could be prudent to omit the `Copy` 
implementation now, to avoid a breaking API change.
