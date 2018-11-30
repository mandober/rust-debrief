# Floats

- type: primitive, scalar, concrete, fixed, sized
- floats have the same endianness on all supported platforms.
- IEEE-754 precisely specifies the bit layout of floats.



## Floats

The IEEE 754-2008 binary32 and binary64 floating-point types: `f32 and f64`


floats | sb | eb | dEmax  | Ebias       | Emin  | Emax | decDig
-------|----|----|--------|-------------|-------|------|-------
`f32`  | 24 |  8 |  38.23 |  2^7−1=127  |  −126 |  127 |   7.22
`f64`  | 53 | 11 | 307.95 | 2^10−1=1023 | −1022 | 1023 |  15.95


f32 | IEEE-754 | ± 3.4 · 10± 38 (~7 digits)
min subnormal: ± 1.401,298,4 · 10-45
min normal: ± 1.175,494,3 · 10-38
max: ± 3.402,823,4 · 1038

f64 | IEEE-754 | ± 1.7 · 10± 308 (~15 digits)
min subnormal: ± 4.940,656,458,412 · 10-324
min normal: ± 2.225,073,858,507,201,4 · 10-308
max: ± 1.797,693,134,862,315,7 · 10308


__binary32 `f32`__
Base: 2
Common name: Single precision
Significand Bits: `24`
Exponent bits: `8`
Decimal digits: 7.22
DecimalE max: 38.23
Exponent bias: 2^7 − 1 = 127
E min: -126
E max:  127

__binary64 `f64`__
Base: 2
Common name: Double precision
Significand Bits: `53`
Exponent bits: `11`
Decimal digits: 15.95
DecimalE max: 307.95
Exponent bias: 2^10 − 1 = 1023
E min: −1022
E max:  1023


range: `10^−308 and 10^308`

The 11 bit width of the exponent allows the representation of numbers between 
10^−308 and 10^308, with full 15–17 decimal digits precision.

By compromising precision, the subnormal representation allows even smaller 
values up to about 5 × 10^−324.


## Specials

__NaN__
Prior to the 2008 version of IEEE-754 interpretation of NaN signaling bit wasn't specified. Most platforms (notably x86 and ARM) picked the interpretation that was ultimately standardized in 2008, but some didn't (notably MIPS). As a result, all signaling NaNs on MIPS are quiet NaNs on x86, and vice-versa.


## Total order and equality

Floats can be compared with the ==, !=, <, <=, >, and >= operators, and with the `partial_cmp()` function; == and != are part of the `PartialEq` trait, while <, <=, >, >=, and `partial_cmp()` are part of the `PartialOrd` trait.

Floats cannot be compared with the `cmp()` function, which is part of the `Ord` trait, as there is no total ordering for floats. Furthermore, there is no total equality relation for floats, and so they also do not implement the `Eq` trait.

There is no total ordering or equality on floats because the floating-point value NaN is not less than, greater than, or equal to any other floating-point value or itself.

Because floats do not implement `Eq` or `Ord`, they may not be used in types whose trait bounds require those traits, such as `BTreeMap` or `HashMap`. This is important because these types assume their keys provide a total ordering or total equality relation, and will malfunction otherwise.

There is a crate that wraps `f32` and `f64` to provide `Ord` and `Eq` implementations, which may be useful in certain cases.


## Partial equality

Why the split between Eq/PartialEq and Ord/PartialOrd?

There are some types in Rust whose values are only partially ordered, or have only partial equality. Partial ordering means that there may be values of the given type that are neither less than nor greater than each other. Partial equality means that there may be values of the given type that are not equal to themselves.

Floating point types (f32 and f64) are good examples of each. Any floating point type may have the value NaN (meaning “not a number”). NaN is not equal to itself (NaN == NaN is false), and not less than or greater than any other floating point value. As such, both f32 and f64 implement PartialOrd and PartialEq but not Ord and not Eq.

As explained in the earlier question on floats, these distinctions are important because some collections rely on total orderings/equality in order to give correct results.


