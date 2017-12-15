# f64
[binary64](https://www.wikiwand.com/en/Double-precision_floating-point_format)

IEEE 754 double-precision binary floating-point format: `binary64`

As with single-precision floating-point format, it lacks precision on integer 
numbers when compared with an integer format of the same size. 

It is commonly known simply as `double`.

name       cn      sb  eb   decDigits decEmax  ExponentBias    Emin  Emax
`binary32  Single  24   8    ≈7(7.22)   38.23    127 =2^7−1    −126   127`
`binary64  Double  53  11  ≈16(15.95)  307.95   1023 =2^10−1  −1022  1023`


The IEEE 754 standard specifies a binary64 as having:
Sign bit: 1 bit
Exponent: `11` bits
Significand precision: `53` bits (52 explicitly stored) *leading bit convention*

**exponent**
The exponent field can be interpreted as either:
- an 11-bit signed integer from −1024 to 1023 (2's complement) or 
- an 11-bit unsigned integer from 0 to 2047, which is the accepted *biased form* 
  in the IEEE 754 binary64 definition. 
  
If the unsigned integer format is used, the exponent value used in the arithmetic 
is the exponent shifted by a bias; for the IEEE 754 binary64 case, an exponent 
value of `1023` represents the actual zero.
i.e. for 2^e−1023 to be 1, e must be 1023

Exponents range from −1022 to +1023 because exponents of −1023 (all 0s) 
and +1024 (all 1s) are reserved for special numbers (±Infinity, 2xNaN).


**significand**
The 53-bit significand precision gives 15-17 significant decimal digits 
precision: 2^−53 ≈ 1.11 × 10^−16

If a decimal number with at most 15 significant digits is converted to f64, 
and then converted back to a decimal with the same number of digits,
the final result should match the original number. 

If a f64 is converted to a decimal number with at least 17 significant digits, 
and then converted back to f64, the final result must match the original number.

2^52 = 4,503,599,627,370,496
2^53 = 9,007,199,254,740,992

In the range from 2^51 to 2^52, the spacing is 0.5, etc.

In the range from 2^52 to 2^53, the spacing is 1, 
so the representable numbers are *integers*.

In the range from 2^53 to 2^54, everything is multiplied by 2, 
so the representable numbers are the *even* ones.


The spacing as a fraction of the numbers in the range from 2^n to 2^n+1 is 2^n−52.
The maximum relative rounding error when rounding a number to the nearest 
representable one (the machine *epsilon*) is therefore `2^−53`.

The 11 bit width of the exponent allows the representation of numbers between 
10^−308 and 10^308, with *full 15–17 decimal digits precision*.

By compromising precision, the *subnormal* representation allows even smaller 
values up to about 5 × 10^−324.