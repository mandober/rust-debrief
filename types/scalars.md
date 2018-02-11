# Scalars

Rust Data Types > Primitives > Scalars  


name         | type          | sample | size    |s|p| trait|m| spec
-------------|---------------|--------|---------|-|-|------|-|-------
boolean      | `bool`        | true   |      1b |s|y| Copy |n|
character    | `char`        | 'ß'    |      4b |s|y| Copy |y|
binary32     | `f32`         | 1.2525 |     32b |s|y| Copy |y|
binary64     | `f64`         | 3.1425 |     64b |s|y| Copy |y|
mach int     | `isize`       | -100   | 32b/64b |s|y| Copy |y|
mach uint    | `usize`       | 100    | 32b/64b |s|y| Copy |y|
int          | `i8-64`       | -42    |   8-64b |s|y| Copy |y|
uint         | `u8-64`       | 42     |   8-64b |s|y| Copy |y|


type  | size | c++                 | js TypedArray
------|-----:|---------------------|-------------------
bool  | 1    | bool                | 
------|------|---------------------|-------------------
i8    | 8    | signed char         | Int8Array
i16   | 16   | signed short int    | Int16Array
i32   | 32   | signed long int     | Int32Array
i64   | 64   |                     |
i128  | 128  |                     |
isize | word | signed int          |
u8    | 8    | unsigned char       | Uint8Array, Uint8ClampedArray
u16   | 16   | unsigned short int  | Uint16Array
u32   | 32   | unsigned long int   | Uint32Array
u64   | 64   |                     |
u128  | 128  |                     |
usize | word | unsigned int        |
------|------|---------------------|-------------------
f32   | 32   | float               | Float32Array
f64   | 64   | double              | Float64Array
------|------|---------------------|-------------------
char  | 4    | wchar_t             |



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
