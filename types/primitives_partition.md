# Categories of primitive types

* Scalar types
  - Boolean
  - Numbers
    * Integers signed and unsigned
    * Machine-architecture dependent integer
    * Floating-point number
  - Character
* Compound types
  - Array
  - Tuple
* Unit type
* Dynamically-sized types
  - Slice
  - String slice
* Primitive pointer types
  - Reference
  - Raw pointer
  - Function pointer


These are some of the ways to partition primitive types into categories.

- Scalars
- Numeric
- Numbers
- Compound (Aggregate) types
- Pointer (reference) types
- Dynamically-sized types


Numeric types:
- integers
- address integers
- floating-points
- boolean
- character



name         | type          | sample | size    |s|p| trait|m| spec
-------------|---------------|--------|---------|-|-|------|-|-------
boolean      | `bool`        | true   |      1b |s|y| Copy |n|
character    | `char`        | 'ß'    |      4b |s|y| Copy |y|
float32      | `f32`         | 1.2525 |     32b |s|y| Copy |y|
float64      | `f64`         | 3.1425 |     64b |s|y| Copy |y|
mach int     | `isize`       | -100   | 32b/64b |s|y| Copy |y|
mach uint    | `usize`       | 100    | 32b/64b |s|y| Copy |y|
int          | `i8-64`       | -42    |   8-64b |s|y| Copy |y|
uint         | `u8-64`       | 42     |   8-64b |s|y| Copy |y|




## Modules

With eponymous module
`i8`        The 8-bit signed integer type.
`u8`        The 8-bit unsigned integer type.
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

With differently named module
`pointer`   Raw, unsafe pointers. (module: `ptr`)
`reference` References, both shared and mutable. (module: `borrow`)
`fn`        Function pointers, like `fn(usize) -> bool`.

Without module
`bool`      The boolean type.
`array`     A fixed-size array, denoted `[T; N]`
`tuple`     A finite heterogeneous sequence, `(T, U, ..)`.


