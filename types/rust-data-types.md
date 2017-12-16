# Rust Data Types

name      | type annot.| crdnl | bits
----------|------------|-------|-----
never     |`!`         | 0     |
unit      |`()`        | 1     |
boolean   |`bool`      | 2     | 1
int8      |`i8`        | 2^2^3 | 8
uint8     |`u8`        | 2^2^3 | 8
int16     |`i16`       | 2^2^4 | 16
uint16    |`u16`       | 2^2^4 | 16
int32     |`i32`       | 2^2^5 | 32
uint32    |`u32`       | 2^2^5 | 32
int64     |`i64`       | 2^2^6 | 64
uint64    |`u64`       | 2^2^6 | 64
int128    |`i128`      | 2^2^7 | 128
uint128   |`u128`      | 2^2^7 | 128
mdi       |`isize`     |       | word
umdi      |`usize`     |       | word
binary32  |`f32`       |       | 32
binary64  |`f64`       |       | 64
character |`char`      |       | 32


name             | type annot.
-----------------|------------
any              |`T`         
top              |`<T:?Sized>`
bottom           |`!`         
array            |`[T; N]`    
tuple            |`(T, U, ..)`
slice            |`[T]`
string slice     |`str`
reference        |`&T`, `&mut T`
raw pointer      |`*const T`, `*mut T`
function pointer |`[unsafe] [extern] [ABI] fn()`



- struct
  * named-fields struct
  * tuple struct
  * unit-like struct
- union
- enum
  * Option<T>
  * Result<T,E>
- smart pointers
  * Box
  * Rc
  * Arc
- function item types
- closure types
- trait objects (dynamically sized type)
- Vector
- String
- HashMap
- Set


