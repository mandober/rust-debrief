# Primitives

Fundamental category of types in any language is the primitives. They are the basic building blocks of language. In Rust, primitives are implemented by the compiler, while the Standard Library implements method on them and provides docs for primitives themselves and for their methods; std also contains several modules with eponymous name as the primitive type - these modules contain less used resources releted to referred primitive, like math constants and such.

Nominal primitives have distinguishing naming style - a single lowercased word, as opposed to CamelCase naming style of other (non-primitive) types. By default, primitives are stored on the stack and they are mostly Copy types.


## Grouping

Primitives can be divided into several groups based on their common properties:
- Numeric types
- Special types
- Primitive Pointer types
- Primitive Compound types
The former two groups are closed i.e. all members of these groups are only found among the primitive types; the latter two groups are in fact subsets of the respective larger group that consist of both, primitive and non-primitive types.


- Numberic types
  - `bool`
  - `char`
  - Numbers
    - Floating-points
      - `f32`
      - `f64`
    - Integer types
      - Integers
        - signed
          - `i8`
          - `i16`
          - `i32`
          - `i64`
          - `i128` __LAB__
        - unsigned
          - `u8`
          - `u16`
          - `u32`
          - `u64`
          - `u128` __LAB__
      - Machine integer
        - `isize` signed
        - `usize` unsigned
- Special types
  - `unit`
  - `never`
- Primitive Pointer types
  - references
  - raw pointers
  - slices
  - string slice
  - function pointer
- Primitive Compound types
  - array
  - tuple



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
- `i128`  128-bit signed integer type. __LAB__
- `u128`  The 128-bit unsigned integer type. __LAB__
- `isize` pointer-sized signed integer type.
- `usize` pointer-sized unsigned integer type.
- `f32`   32-bit floating point type.
- `f64`   64-bit floating point type.

Compound
- array: a fixed-size array, denoted `[T; N]`
- tuple: a finite heterogeneous sequence, `(T, U, ..)`.

Pointer types
- `fn` function pointers, like `fn(usize) -> bool`.
- raw pointers: unsafe pointers, `*const T` and `*mut T`.
- references: shared and mutable references, `&T` and `&mut T`
- slice, `[T]`, always seen in the borrowed form, `&[T]`
- string slice, `str`, always seen in the borrowed form, `&str`

Special
- unit: the unit type, `()`.
- never: the never type, `!`. __LAB__ 
