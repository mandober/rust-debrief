# Raw pointers

- raw pointers, `*const T` and `*mut T`, since 1.0.0, [online docs][ptr]
- primitive, derived type; raw ptr and raw mutable ptr
- same as C pointers; no safety or aliveness guarantees
- bind it to a lifetime asap
- deref of ptr requires unsafe block
- get pointers by coercing a (mut) ref; consuming a `Box`; from C
- cast a const ptr to get a mut ptr
- create NULL pointer with `null` and `null_mut` fn
- use `is_null` to check for NULL pointer
- use `offset` for pointer arithmetic

[ptr]: https://doc.rust-lang.org/std/primitive.pointer.html


## NULL pointers
Unsized types have many possible null pointers, as only the raw data pointer is considered, not their length, vtable, etc. Therefore, two pointers that are null may still not compare equal to each other.





## Coercing a reference to pointer
Coercing a reference to pointer does not take ownership of the original allocation and requires no resource management later (but beware of dangling pointer i.e. using it after its lifetime).

```rust
// coerce a ref to get const pointer
let n = 10;
let ptr: *const i32 = &n;
//   let the ptr's base type be inferred:
let ptr: *const _ = &n;

// get mut ptr by casting a const ptr:
let mptr = ptr as *mut i32;
//   or with type (partially) annotated
let mptr: *mut _ = ptr as *mut i32;
//   or get mut pointer by coercing a mut ref
let mptr: *mut _ = &mut 88;

// get a pointer to a boxed value
let boxed = Box::new(10_i32);
//   first deref the box, then coerce a ref
let ptr: *const i32 = &*boxed;
```


## Consume a box to get a pointer
The `into_raw` function consumes a box and returns the raw pointer.
It doesn't destroy `T` or deallocate any memory.

```rust
let boxed: Box<i32> = Box::new(88);
let boxed: *mut i32 = Box::into_raw(boxed);
// By taking ownership of the original `Box<T>`, we should
// feel obligated to put it together later to be destroyed
unsafe {
    drop(Box::from_raw(boxed));
    // drop call for clarity: indicates that we are
    // done with the value and it should be destroyed
}
```


## Get pointer from C
Usually you wouldn't literally use `malloc` and `free` from Rust, but C is a common source of pointers:

```rust
extern crate libc;
use std::mem;

fn main() {
  unsafe {
    // malloc
    let n: *mut i32 = libc::malloc(mem::size_of::<i32>()) as *mut i32;
    // check NULL
    if n.is_null() { panic!("out of memory (OOM)"); }
    // free
    libc::free(n as *mut libc::c_void);
  }
}
```

