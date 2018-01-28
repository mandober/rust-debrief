# Primitives

- `bool` [Boolean](../bool.md) - the boolean type


- `i8`        The 8-bit signed integer type.
- `u8`        The 8-bit unsigned integer type.

- Integers signed and unsigned
- Machine-architecture dependent integer
- Floating-point number
- Character
- Array
- Tuple
- Slice
- String slice
- Reference
- Raw pointer
- Function pointer

## List of primitives

`i16`       The 16-bit signed integer type.
`u16`       The 16-bit unsigned integer type.
`i32`       The 32-bit signed integer type.
`u32`       The 32-bit unsigned integer type.
`i64`       The 64-bit signed integer type.
`u64`       The 64-bit unsigned integer type.
`i128`      The 128-bit signed integer type. [LAB]
`u128`      The 128-bit unsigned integer type. [LAB]
`isize`     The pointer-sized signed integer type.
`usize`     The pointer-sized unsigned integer type.
`f32`       The 32-bit floating point type.
`f64`       The 64-bit floating point type.
`char`      A character type.
`slice`     A dynamically-sized view into a contiguous sequence.
`str`       String slices.
`pointer`   Raw, unsafe pointers. (module: `ptr`)
`reference` References, both shared and mutable. (module: `borrow`)
`fn`        Function pointers, like `fn(usize) -> bool`.
`array`     A fixed-size array, denoted `[T; N]`
`tuple`     A finite heterogeneous sequence, `(T, U, ..)`.

