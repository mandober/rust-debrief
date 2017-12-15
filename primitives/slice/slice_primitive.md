# Primitive Type `str` 1.0.0
https://doc.rust-lang.org/stable/std/primitive.slice.html

A dynamically-sized view into a contiguous sequence, `[T]`.

Slices are a view into a block of memory represented as a pointer and a length.

Their size does not have to be known at compile time

Slices are either mutable or shared:
- shared slice type is `&[T]`
- mutable slice type is `&mut [T]`


```rust
// slicing a Vec
let vec = vec![1, 2, 3];
let int_slice = &vec[..];

// coercing an array to a slice
let str_slice: &[&str] = &["one", "two", "three"];
```
Slices are either mutable or shared. The shared slice type is &[T], while the mutable slice type is &mut [T], where T represents the element type. 
For example, you can mutate the block of memory that a mutable slice points to:

```rust
let x = &mut [1, 2, 3];
x[1] = 7;
assert_eq!(x, &[1, 7, 3]);
```
