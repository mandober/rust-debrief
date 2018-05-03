# Primitive types: numbers

- type: primitive, scalar, sized, fized, concrete 
- suffix type annotation for number literal.
- underscore as visual grouping/separation character (e.g. `1_000_u32`)
- Signed integers use two's compliment representation.
- Floats use IEEE-754 representation.
- No minus sign: no negative number literal, only negation operator;   
  Only signed ints and floats support it.
- Total of 14 types represent integral and rational numbers.
- Integrals (12 types):
  - signed (5): `i8`, `i16`, `i32`, `i64`, `i128`
  - unsigned (5): `u8`, `u16`, `u32`, `u64`, `u128`
  - machine-dependent integers (2 types): `isize`, `usize`
- Floating-points (2): `f32`, `f64`
- ints and floats have the same endianness on all supported platforms.
- IEEE-754 precisely specifies the bit layout of floats.


## Numbers
In any language primitive numbers are the most basic type. They are basic building blocks of language. They are wholesome types; they are scalars - they cannot be broken into smaller parts (unlike coumpound types). Numbers in Rust are divided into two groups: integers and floats.

__No negative numbers__   
Rust dosn't have a minus sign, so there's no negative number literal. There's only negation operator. Only signed integers and floats support negation. It is an error to apply negation to unsigned number types.


## Classification

By bit-width:
- 8 bits: `i8`, `u8`
- 16 bits: `i16`, `u16`
- 32 bits: `i32`, `u32`, `f32`, (`isize` @x86, `usize` @x86)
- 64 bits: `i64`, `u64`, `f64`, (`isize` @x86_64, `usize` @x86_64)
- 128 bits: `i128`, `u128` (experimental)

Numbers in the inclusive range 0-127 can be represented using any of the 14 available number types.


## Type annotation
Number literals have additional way of annotating the type: a suffix annotation. The same type name is suffixed to the number literal. Underscore character can be used to visually group/separate digits.


```rust
// usual
let n: f32 = 27.6;

// suffix
let arr = [1_u8, 2, 3];

// redundant
let n: u16 = 0x100_u16;

// underscore
let pretty = 1_000_000_u32;
```

# Type inference
The type of an unsuffixed literal is determined by type inference:
- if the type can be uniquely determined from the surrounding program context, the unsuffixed literal has that type.
- if the program context under-constrains the type, an integer defaults to `i32` and a float defaults to `f64`.
- If the program context over-constrains the type, it is considered a static type error.

