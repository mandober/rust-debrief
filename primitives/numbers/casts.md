# Casts

1. `int <=> int`: between two integers of the same size (u8 -> i8) 
    is a no-op. In computation is noop, representation differs.
2. `INT => int`: from larger to smaller integer
    - if both unsigned or positive: *modulo*
    - otherwise: *truncate*
3. `int => INT`: from smaller to larger integer will
    - *zero-extend* if the source is unsigned
    - *sign-extend* if the source is signed


- `f2i`: from a float to an integer will *round* the float towards zero
  NOTE: currently this will cause Undefined Behavior if the rounded value cannot be represented by the target integer type. This includes Inf and NaN. This is a bug and will be fixed.
- Casting from an integer to float will produce the floating point 
  representation of the integer, rounded if necessary (rounding strategy unspecified)
- Casting from an f32 to an f64 is perfect and lossless
- Casting from an f64 to an f32 will produce the closest possible value
  (rounding strategy unspecified)
  NOTE: currently this will cause Undefined Behavior if the value is finite but larger or smaller than the largest or smallest finite value representable by f32. This is a bug and will be fixed.


```rust
// i <=> u
let t =  255_u8 as i8; //  -1
let t = -128_i8 as u8; // 128

// UI => ui
let t = 65535_u16 as u8; // 255
let t = 65535_u16 % 256; // 255

let t = -510_i16 as u8;  //   2
let t =  510_i16 as u8;  // 254
let t =  511_i16 as u8;  // 255
let t =  512_i16 as u8;  //   0

let t =  130_i16 as i8;  //-126
let t =  129_i16 as i8;  //-127
let t =  128_i16 as i8;  //-128
let t =  127_i16 as i8;  // 127 -|
let t =   -2_i16 as i8;  //  -2  |
let t = -127_i16 as i8;  //-127  |
let t = -128_i16 as i8;  //-128 -|
let t = -129_i16 as i8;  // 127
let t = -130_i16 as i8;  // 126

let t = -510_i16 as i8;  //   2
let t =  510_i16 as i8;  //  -2

let t =  254_i16 as i8;  //  -2
let t =  255_i16 as i8;  //  -1
let t =  256_i16 as i8;  //   0
let t = -256_i16 as i8;  //   0
let t = -255_i16 as i8;  //   1
```
