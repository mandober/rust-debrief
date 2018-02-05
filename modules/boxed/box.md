# Box

- `struct Box<T>`
- path: `std::boxed::Box`
- Box is somewhat of a primitive, it is a special type in the compiler.
- box is an owned reference: a fat pointer to data on the heap



## `boxed` module

- Module `std::boxed`, since 1.0.0
- [doc](https://doc.rust-lang.org/std/boxed "external link:std docs")
- Structs:
  - `Box` pointer type for heap allocation.
  - `ExchangeHeapSingleton`[LAB] This the singleton type for `boxed::HEAP`.
  - `IntermediateBox`[LAB] uninitialized backing storage for Box.
- Constants:
  - `HEAP` [LAB] A value that represents the heap.
- Traits:
  - `FnBox` [LAB] version of the FnOnce for use with boxed closure objects.


**Structs**
- `Box` - A pointer type for heap allocation.
- `ExchangeHeapSingleton` - [LAB] This the singleton type used solely for `boxed::HEAP`.
- `IntermediateBox` - [LAB] IntermediateBox represents uninitialized backing storage for Box.

**Constants**
- `HEAP` - [LAB] A value that represents the heap. This is the default place that the box keyword allocates into when no place is supplied.

**Traits**
- `FnBox` - [LAB] `FnBox` is a version of the `FnOnce` intended for use with boxed closure objects. The idea is that where one would normally store `Box<FnOnce()>` in a data structure, you should use `Box<FnBox()>`. The two traits behave essentially the same, except that a `FnBox` closure can only be called if it is boxed. Note that `FnBox` may be deprecated in the future if `Box<FnOnce()>` closures become directly usable.


## Box struct

The main feature of `boxed` module is `Box<T>` struct, a "box" that provides the simplest form of heap allocation. Box provides ownership for this allocation, and drops its contents when it goes out of scope.

Since box has a known, defined, size it is frequently used "to box" a recursive type or a type of unknown size.


```rust
#[lang = "owned_box"]
// pub struct Box<T: ?Sized>(Unique<T>);
pub struct Box<T: ?Sized>(_);
```

Creating a box:

```rust
let x = Box::new(5);
// or in nightly with `box_syntax` feature flag
#![feature(box_syntax, box_patterns)]
let x = box 5;
```


## Box is special
https://manishearth.github.io/blog/2017/01/10/rust-tidbits-box-is-special

Box is somewhat of a primitive, it is a special type - the compiler has intimate knowledge about it. Because of this the box can move out of a borrow:

```rust
// boxed vec (redundant type annotations)
let x: Box<Vec<_>> = Box::new(vec![1,2,3,4]);
// DerefMove
// moves the vec out into y, then deallocates the box
// but does not call a destructor on the vec
let y: Vec<_> = *x;

// for any other type
let v = &vec![1,2,3,4];
// it gives an error:
let m = *v;
// ERROR: cannot move out of borrowed content
```

For a regular type, `*x` will produce a temporary that must be immediately borrowed or copied. You cannot do `let x = *y` for a non-Copy type, such an operation will produce a "cannot move out of a borrow" error. This dereference operation will call `DerefMut::deref_mut` or `Deref::deref` based on how it gets borrowed.



### Deref and DerefMut

`Deref::deref` trait is for defining how a type should be derefrenced. Since box is special, its deref implementation is:

```rust
impl<T: ?Sized> Deref for Box<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &**self
    }
}

impl<T: ?Sized> DerefMut for Box<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut **self
    }
}
```

`deref` returns `&**self` - since `self` is a `&Box<T>`, dereferencing it once will provide a `Box<T>`, and the second dereference will dereference the box to provide a `T`. We then wrap it in a reference and return it.


Implementing `Deref` for smart pointers makes accessing the data behind them convenient, which is why they implement `Deref`. On the other hand, the rules regarding `Deref` and `DerefMut` were designed specifically to accommodate smart pointers. Because of this, `Deref` should only be implemented for smart pointers to avoid confusion.
