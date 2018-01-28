# Primitive: Pointer
https://doc.rust-lang.org/std/primitive.pointer.html

Primitive Type *pointer* 1.0.0

Raw, unsafe pointers, `*const T`, and `*mut T`.

Working with raw pointers in Rust is uncommon,
typically limited to a few patterns.

Use the `null` and `null_mut` functions to create null pointers,
and the `is_null` method of the `*const T`, and `*mut T` types to check for null.
The `*const T`, and `*mut T` types also define the offset method, for pointer math.

Common ways to create raw pointers
1. Coerce a reference `&T` or mutable reference `&mut T`
2. Consume a box `Box<T>`
3. Get it from C


## Coerce a reference `&T` or mutable reference `&mut T`

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
This does not take ownership of the original allocation and requires no resource 
management later, but you must not use the pointer after its lifetime.

