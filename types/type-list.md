# Types

1. `bool`  boolean
2. `char`  character
3. `i8`    8-bit signed integer type.
4. `u8`    8-bit unsigned integer type.
5. `i16`   16-bit signed integer type.
6. `u16`   16-bit unsigned integer type.
7. `i32`   32-bit signed integer type.
8. `u32`   32-bit unsigned integer type.
9. `i64`   64-bit signed integer type.
10. `u64`   64-bit unsigned integer type.
11. `i128`  128-bit signed integer type. [Experimental]
12. `u128`  The 128-bit unsigned integer type. [Experimental]
13. `isize` pointer-sized signed integer type.
14. `usize` pointer-sized unsigned integer type.
15. `f32`   32-bit floating point type.
16. `f64`   64-bit floating point type.
17. array: a fixed-size array, denoted [T; N]
18. tuple: a finite heterogeneous sequence, (T, U, ..).
19. vector: a growable array type stored on the heap, `Vec<T>`
20. unit: the unit type, `()`.
21. never: the never type, `!`. [Experimental] 
22. `fn` function pointers, like `fn(usize) -> bool`.
23. raw pointers: unsafe pointers, `*const T` and `*mut T`.
24. references: shared and mutable references, `&T` and `&mut T`
25. slice, `[T]`, always seen in the borrowed form, `&[T]`
26. string slice, `str`, always seen in the borrowed form, `&str`
27. string, `String`
28. box, `Box<T>`



Scalars: 1-16
Compound: 17-19
Special:20, 21
Primitive pointer types: 22-26
Pointer types, smart pointers: 27, 28
