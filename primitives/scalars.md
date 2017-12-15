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


type  | size | c++
------|-----:|---------------------
bool  | 1    | bool
------|------|---------------------
i8    | 8    | signed char
i16   | 16   | signed short int
i32   | 32   | signed long int
i64   | 64   | 
i128  | 128  | 
isize | word | signed int
u8    | 8    | unsigned char
u16   | 16   | unsigned short int
u32   | 32   | unsigned long int
u64   | 64   | 
u128  | 128  | 
usize | word | unsigned int
------|------|---------------------
f32   | 32   | float
f64   | 64   | double
------|------|---------------------
char  | 4    | wchar_t


