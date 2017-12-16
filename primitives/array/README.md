# Array

- name: array
- [std doc](https://doc.rust-lang.org/std/primitive.array.html)
- primitive, compound, generic type
- no accompanying module
- contiguous, fixed-size, sequence of homogenous elements
- structural type, denoted `[T; N]`, for a constant size, N>=0.  
  N is the length of the array, a part of its type.  
  N is non-negative compile-time constant.  
  N (like all numbers for index, size and length) is `usize`.
- subtypes: the number of all array types is a product of all available Rust types and all sizes: `T × usize`
- shared reference to array: `&[T; N]`
- mutable reference to array: `&mut [T; N]`
- can be coerced to slice `[T]`
