# Numbers

- numbers are the most primitive types

- Numbers:
  - Integers
    - signed:   `i8`, `i16`, `i32`, `i64`, `i128`
    - unsigned: `u8`, `u16`, `u32`, `u64`, `u128`
  - Machine-dependent integers: `isize`, `usize`
  - Floating-points: `f32`, `f64`



## Categories
* signedness:
  - signed:   `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `f32`, `f64`
  - unsigned: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
* bit-width: 8, 16, 32, 64, 128
* dependency: independent and machine-architecture dependent
* fixed and floating-point



## Types
Numbers are partitioned into 3 categories and they have 14 subtypes
- integers: 10 subtypes
- machine architecture dependent integers: 2 subtypes
- floating-points: 2 subtypes

For example, number one (as all integers 0-127) can take any of the 14 different types: `1_i8, 1_u8, 1_i16, 1_u16, 1_i32, 1_u32, 1_i64, 1_u64, 1_i128, 1_u128, 1_isize, 1_usize, 1_f32, 1_f64`

¹2 ²2 ³2 ⁴2 ⁵2 ⁶2 ⁷2 ⁸2 ⁹2

⁴2 = 2^^4 = 2¹⁶ = 2^16



Numbers by bit-width:

exp:   | 2^3 | 2^4 |  2^5  |  2^6  | 2^7
-------|-----|-----|-------|-------|-------
int    |`i8` |`i16`|`i32`  |`i64`  |`i128`
uint   |`u8` |`u16`|`u32`  |`u64`  |`u128`
float  |     |     |`f32`  |`f64`  |
isize  |     |     | x32   | x64   |
usize  |     |     | x32   | x64   |

```
2^8   = 2^(2^3) =
2^16  = 2^(2^4) = 2^(2^(2^2)) = ⁴2 = 2^^4
2^32  = 2^(2^5)
2^64  = 2^(2^6)
2^128 = 2^(2^7)
```
