# Box

- box struct `Box<T>`, `std::boxed::Box`
- box is a smart pointer, a smart type of reference that owns its data; it has a fat pointer on the stack to its owned data on the heap.
- box provides the simplest form of heap allocation; box provides ownership for this allocation, and drops its contents when it goes out of scope.
- since box has a known, defined, size it is frequently used "to box" a recursive type or a type of unknown size.
- box is somewhat of a primitive, it is a special type - the compiler has intimate knowledge about it; due to this the box can move out of a borrow.


## Boxes are most often used:
- When you have a type whose size can't be known at compile time, and you want to use a value of that type in a context that needs to know an exact size.
- When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so.
- When you want to own a value and only care that it's a type that implements a particular trait rather than knowing the concrete type itself.


## Box definition in std:

```rust
#[lang = "owned_box"]
// pub struct Box<T: ?Sized>(Unique<T>);
pub struct Box<T: ?Sized>(_);
```

## Creating a box:

```rust
fn main() {
    let x = Box::new(5);

} // boxed is dropped here:
// The deallocation happens for both the box (stored on the stack) 
// and the data it points to (stored on the heap).
```

More convenient way to create a boxed value - available only in nightly Rust releases with `box_syntax` feature flag enabled:

```rust
#![feature(box_syntax)]
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

For a regular type, `*x` will produce a temporary value that must be immediately borrowed or copied. You cannot do `let x = *y` for a non-Copy type, such an operation will produce a "cannot move out of a borrow" error. This dereference operation will call `DerefMut::deref_mut` or `Deref::deref` based on how it gets borrowed.



### Deref and DerefMut

Implementing `Deref` for smart pointers makes accessing the data behind them convenient, which is why they implement `Deref`. Along with `Drop`, it is the distinguishing characteristics of smart pointer from ordinary structs. 

In fact, the rules regarding `Deref` and `DerefMut` were designed specifically to accommodate smart pointers, and because of this, `Deref` should only be implemented for them in order to avoid confusion.

Box implements the `Deref` trait, which allows `Box<T>` values to be treated like references. When a `Box<T>` value goes out of scope, the heap data that the box is pointing to is cleaned up as well because of the `Box<T>` type's `Drop` trait implementation. 

Box's impl of deref:

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

