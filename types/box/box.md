# box

name: box
anot: `Box<T>`
type: composite type, generic type, reference types group
mod: `std::boxed`
sized: yes
store: ptr on stack to data on heap
code: `let b = Box::new(5)`

Box is a struct defined in `std::boxed::Box`.

Box is somewhat of a primitive, it is a special type in the compiler.

CAN move out of a borrow:

```rust
// annotate a box:
let x: Box<Vec<_>> = Box::new(vec![1,2,3,4]);
// moves vec out of box
let _y: Vec<_> = *x;
```



## Box is special
https://manishearth.github.io/blog/2017/01/10/rust-tidbits-box-is-special

Box is somewhat of a primitive type.
For a regular type, `*foo` will produce a temporary that must be immediately borrowed or copied. You cannot do `let x = *y` for a non-Copy type. This dereference operation will call `DerefMut::deref_mut` or `Deref::deref` based on how it gets borrowed. With `Box<T>`, you can do this:

```rust
let x = Box::new(vec![1,2,3,4]);
let y = *x; // moves the vec out into y, then deallocates the box
            // but does not call a destructor on the vec
```

For any other type, such an operation will produce a "cannot move out of a borrow" error.

The current status is that Box<T> is still a special type in the compiler. 