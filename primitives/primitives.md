# Primitives

Fundamental type category in any language are the primitives. Primitive types are the basic building blocks of a language. In Rust, they are implemented by the compiler, while std implements method on them. Nominal primitives have distinguishing naming style in comparison to other types - a single lowercased word, as opposed to CamelCase naming style of non-primitives. Primitives are stored on the stack.

Rust's primitives: numbers, boolean, character, reference, raw pointer, function pointer, slice, string slice, array, tuple, unit, and never type.

Scalars
- `bool`  boolean
- `char`  character
- `i8`    8-bit signed integer type.
- `u8`    8-bit unsigned integer type.
- `i16`   16-bit signed integer type.
- `u16`   16-bit unsigned integer type.
- `i32`   32-bit signed integer type.
- `u32`   32-bit unsigned integer type.
- `i64`   64-bit signed integer type.
- `u64`   64-bit unsigned integer type.
- `i128`  128-bit signed integer type. [Experimental]
- `u128`  The 128-bit unsigned integer type. [Experimental]
- `isize` pointer-sized signed integer type.
- `usize` pointer-sized unsigned integer type.
- `f32`   32-bit floating point type.
- `f64`   64-bit floating point type.

Compound
- array: a fixed-size array, denoted [T; N]
- tuple: a finite heterogeneous sequence, (T, U, ..).

Pointer types
- `fn` function pointers, like `fn(usize) -> bool`.
- raw pointers: unsafe pointers, `*const T` and `*mut T`.
- references: shared and mutable references, `&T` and `&mut T`
- slice, `[T]`, always seen in the borrowed form, `&[T]`
- string slice, `str`, always seen in the borrowed form, `&str`

Special
- unit: the unit type, `()`.
- never: the never type, `!`. [Experimental] 
