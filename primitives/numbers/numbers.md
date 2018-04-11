# Numbers


- numbers are scalar types, the most primitive types
- integers vary in bit-width and signedness
- machine-dependent integers vary in signedness, but their bit-width is determined by the host architecture: 32 bits on x86, 64bits on x86_64
- floats vary in precision

- There are 14 number types representing integral and rational numbers.
- Integrals:
  - Integers (10): `i/u8`, `i/u16`, `i/u32`, `i/u64`, `i/u128`
  - Machine-dependent integers (2): `isize`, `usize`
- Floating-points (2): `f32`, `f64`
- Categories:
  * Signedness:
    - signed (8): `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `f32`, `f64`
    - unsigned (6):  `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
  * Bit-width:
    - 8 bits: `i8`, `u8`
    - 16 bits: `i16`, `u16`
    - 32 bits: `i32`, `u32`, `f32`, (`isize` @x86, `usize` @x86)
    - 64 bits: `i64`, `u64`, `f64`, (`isize` @x86_64, `usize` @x86_64)
    - 128 bits: `i128`, `u128` (Experimental type)
- Signed integers use two'scompliment representation.
- Floats use IEEE-754 representation.


* For example, number 1 can take any of the 14 number types.
* There is *no minus sign* i.e. no negative number literal, only negation operator. Only signed integers and floating-points support negation. It is an error to apply negation to unsigned types.

