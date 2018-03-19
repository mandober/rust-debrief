# Box

- box struct `Box<T>`, `std::boxed::Box`
- box is a smart pointer, a smart type of reference that owns its data; it has a fat pointer on the stack to its owned data on the heap.
- box provides the simplest form of heap allocation; box provides ownership for this allocation, and drops its contents when it goes out of scope.
- since box has a known, defined, size it is frequently used "to box" a recursive type or a type of unknown size.
- box is somewhat of a primitive, it is a special type - the compiler has intimate knowledge about it; due to this the box can move out of a borrow.


## boxed
- module: `std::boxed`, since 1.0.0
- module [doc](https://doc.rust-lang.org/std/boxed "external link:std docs")
- the main feature of this module is [Box struct](box.md).
- `boxed` module contains
    - Structs:
    - `Box`
    - `ExchangeHeapSingleton`[LAB]
    - `IntermediateBox`[LAB]
    - Constants:
    - `HEAP` [LAB]
    - Traits:
    - `FnBox` [LAB]

### Structs
- `Box` a pointer type for heap allocation.
- `ExchangeHeapSingleton` [LAB] the singleton type used for `boxed::HEAP`.
- `IntermediateBox` - [LAB] uninitialized backing storage for Box.

### Constants
- `HEAP` [LAB] a value that represents the heap. This is the default place that the box keyword allocates into when no place is supplied.

### Traits
- `FnBox` [LAB] a version of the `FnOnce` intended for use with boxed closure objects. `Box<FnBox()>` is to be used instead of storing `Box<FnOnce()>` in a data structure. The two traits behave essentially the same, except that a `FnBox` closure can only be called if it is boxed. Note that `FnBox` may be deprecated in the future if `Box<FnOnce()>` closures become directly usable.

The main feature of this module is [Box struct](box.md).



## Boxes are most often used:
- When you have a type whose size can't be known at compile time, and you want to use a value of that type in a context that needs to know an exact size.
- When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so.
- When you want to own a value and only care that it's a type that implements a particular trait rather than knowing the concrete type itself.

The deallocation happens for both the box (stored on the stack) and the data it points to (stored on the heap).


## Box definition in std

```rust
#[lang = "owned_box"]
// pub struct Box<T: ?Sized>(Unique<T>);
pub struct Box<T: ?Sized>(_);
```



## Manipulating boxes

Currently, the only stable way to create a box is the `Box::new()` method. 

```rust
const BOX: &'static str = "there is no box";
let opt_box = Some(Box::new(BOX));

match opt_box {
    None => println!("no box found: {}", BOX),
    Some(boxed_value) => {
        // `boxed_value` has the type `Box<&str>`
        // box can be derefrenced explicitly,
        // although `Box<T>` derefs to `T` anyway.
        let unbox: &str = *boxed_value;
        println!("{}. {}. {}.", BOX, unbox, boxed_value);
    },
}
```

More convenient syntax is available in nightly Rust release under the feature flag `#![feature(box_syntax)]`.

Also, it is not possible to destructure a box in stable Rust, but the feature flag `#![feature(box_patterns)]` is available in nightly that allows destructuring a box in the match block pattern. Similarly to `ref`, the unstable `box` keyword can be used for both purposes, to create a box and to destructure it.

```rust
#![feature(box_patterns, box_syntax)]
const BOX: &'static str = "there is no box";

let xboxed = Some(box BOX);

match xboxed {
  None        => println!("do not try and destructure the box"),
  Some(box x) => println!("instead try to realize the truth: {}", x),
}
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

