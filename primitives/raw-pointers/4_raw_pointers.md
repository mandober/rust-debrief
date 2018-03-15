# Raw pointers


The main non-composite, derived type is the **pointer**, a data type whose value refers directly ("points to") to another value stored elsewhere in the memory using its address. It is a primitive kind of reference.

Raw pointer, `*const T` or mutable raw pointer, `*mut T`, are pointers without safety or aliveness guarantees. Copying or dropping a raw pointer has no effect on the lifecycle of any other value. Dereferencing a raw pointer or converting it to any other pointer type is an unsafe operation. Raw pointers are generally discouraged in Rust code. They exist to support interoperability with foreign code, and writing performance critical or low-level functions.

Raw pointers are written as `*const T` or `*mut T`, for example `*const i32` means a raw pointer to a 32-bit integer.

- https://doc.rust-lang.org/std/primitive.pointer.html
- primitive Type *pointer* 1.0.0
- Raw, unsafe pointers, `*const T`, and `*mut T`.

Use the `null` and `null_mut` functions to create null pointers, and the `is_null` method of the `*const T`, and `*mut T` types to check for null. The `*const T`, and `*mut T` types also define the offset method, for pointer math.

Common ways to create raw pointers
1. Coerce a reference `&T` or mutable reference `&mut T`
2. Consume a box `Box<T>`
3. Get it from C



## Coercing a reference to pointer

Coercing a reference to pointer does not take ownership of the original allocation and requires no resource management later, but you must not use the pointer after its lifetime.

```rust
let my_num: i32 = 10;
let my_num_ptr: *const i32 = &my_num;
let mut my_speed: i32 = 88;
let my_speed_ptr: *mut i32 = &mut my_speed;Run
```

To get a pointer to a boxed value, dereference the box:

```rust
let my_num: Box<i32> = Box::new(10);
let my_num_ptr: *const i32 = &*my_num;

let mut my_speed: Box<i32> = Box::new(88);
let my_speed_ptr: *mut i32 = &mut *my_speed;
```
