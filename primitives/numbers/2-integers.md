# Integers

Integers are primitive scalar types.

The type of an unsuffixed integer literal is determined by type inference:
- if an integer type can be uniquely determined from the surrounding program
  context, the unsuffixed integer literal has that type.
- if the program context under-constrains the type, it defaults to i32
- If the program context over-constrains the type, 
  it is considered a static type error.

```rust
‭// 65424‬ dec in bin:
0b_1111_1111_1001_0000;     // type i32
0b_1111_1111_1001_0000_i32; // type i32

// invalid suffix
0invalidSuffix; // error!

// uses numbers of the wrong base
123AFB43; // error! AFB are not decimal digits
0b_0102;   // error! 2 is not binary digit
0o_0581;   // error! 8 is not octal digit

// integers too big for their type: overflow error
128_i8; // error!
256_u8; // error!

// bin, hex and octal literals must have at least one digit:
0b_;    // error!
0b____; // error!

-1i8; // application of unary minus operator, same as:
-(1i8);
```
Note that the Rust syntax considers `-1i8` as an application of the unary minus operator to an integer literal `1i8`, rather than a single integer literal.



# Signedness and bit-width
Integers can be divided by signedness into signed and unsigned,
and by bit-width into: 8, 16, 32, 64 and 128 bit-width.
(128-wide subtype is still experimental as of Nov-2017)

The signedness of each type is the letter in their type signature (i or u)
The bit-width of each type is the number in their type signature (8,16,32,64,128).

These types can represent 2 to the power of signum *different values*
`2^s`, where s is the bit-width stated in signum (8, 16, 32, 64, 128) i.e.
`2^(2^w)`, where w is the bit-width exponent of exponent, 3-7 inclusive.


**Value Ranges, `positive bias`**
i8:                        -128 *to* 127
u8:                           0 *to* 256
i16:                    -32,768 *to* 32,767
u16:                          0 *to* 65,563
i32:             -2,147,483,648 *to* 2,147,483,647
u32:                          0 *to* 4,294,967,296
i64: -9,223,372,036,854,775,808 *to* 9,223,372,036,854,775,807
u64:                          0 *to* 18,446,744,073,709,551,616
u128                   -(2^127) *to* 1.7014118346046923173168730371588e+38 (2^127)


**Value Ranges, `negative bias`**
i64: -9,223,372,036,854,775,808 *to* 9,223,372,036,854,775,807
i32:             -2,147,483,648 *to* 2,147,483,647
i16:                    -32,768 *to* 32,767
i8:                        -128 *to* 127
u8:                           0 *to* 256
u16:                          0 *to* 65,563
u32:                          0 *to* 4,294,967,296
u64:                          0 *to* 18,446,744,073,709,551,616
u128                          0 *to* 3.4028236692093846346337460743177e+38
f64:          e−308 (nearing 0) *to* e308



**Number of different representable values**
int8  = 2^8  = 2^(2^3) = 256
int16 = 2^16 = 2^(2^4) = 65,563
int32 = 2^32 = 2^(2^5) = 4,294,967,296
int64 = 2^64 = 2^(2^6) = 18,446,744,073,709,551,616
int128= 2^128= 2^(2^7) = 3.4028236692093846346337460743177e+38




## Unsigned +
The unsigned types are positive integers (and zero), ranging from 0 to
2 to the power of their signum, minus one: `from 0 to 2^s - 1` or
mathematically ascribed as: `[0 - 2^s)`, where first set member (zero) 
is inluded, but the last set member is not.

word | range      | windows api name
-----|------------|-------------------
u8   | [0, 2^8)   | Byte
u16  | [0, 2^16)  | Word
u32  | [0, 2^32)  | DWord (double word)
u64  | [0, 2^64)  | QWord (quad word)
u128 | [0, 2^128) | OWord (octal, double-quad word)


u8  = 2^8  = 2^(2^3) = 0 - 256
u16 = 2^16 = 2^(2^4) = 0 - 65,563
u32 = 2^32 = 2^(2^5) = 0 - 4,294,967,296
u64 = 2^64 = 2^(2^6) = 0 - 18,446,744,073,709,551,616
u128= 2^128= 2^(2^7) = 0 - 3.4028236692093846346337460743177e+38



## Signed ±
Signed integers are represented using *two's complement*.

The signed types range from minus
2 to the power of signature minus 1 to
2 to the power of signature minus 1, minus 1;
±((2^(s-1))(-1))
from -(2^(s-1)) to ((2^(s-1))-1)

The signed two's complement word types `i8, i16, i32 and i64`, 
with values drawn from the integer intervals 
[-(2^7) , 2^7)  =                       -128 *to* 127
[-(2^15), 2^15) =                    -32,768 *to* 32,767
[-(2^31), 2^31) =             -2,147,483,648 *to* 2,147,483,647
[-(2^63), 2^63) = -9,223,372,036,854,775,808 *to* 9,223,372,036,854,775,807
[-(2^127), 2^127)