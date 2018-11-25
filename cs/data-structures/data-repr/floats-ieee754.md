# IEEE Standard for Floating-Point Arithmetic (IEEE 754)

## Format

- Two infinities: +∞ and −∞.
- Two NaN: a quiet NaN (qNaN) and a signaling NaN (sNaN).
- Finite numbers: which may be either base 2 (binary) or base 10 (decimal).

Each finite number is described by 3 integers and base:
s = sign (zero or one): 0 (positive) or 1 (negative)
c = significand or coefficient
q = exponent
b = base or radix: 2 or 10

The numerical value of a finite number is `(−1)^s × c × b^q`
± c × 2^q   ± c × 10^q

For example, if the base is 10, the sign 1 (indicating negative),
the significand 12345, and the exponent −3, then the value is 
(−1)^1 × 12345 × 10^−3 = −1 × 12345 × 0.001 = −12.345

For example, if the base is 2, the sign 1 (indicating negative),
the significand 12345, and the exponent −3, then the value is 
(−1)^1 × 12345 × 2^−3 = −1 × 12345 × 0.125 = −1543.125


The possible finite values that can be represented in a format are determined by 
the base b, the number of digits in the significand (precision `p`), 
and the exponent parameter `emax`:
- `c` must be an integer in the range zero through `b^p−1`
  (e.g., if b=10 and p=7 then c is 0 through 9999999)
- `q` must be an integer such that `1−emax` ≤ `q+p−1` ≤ `emax`
  (e.g., if p=7 and emax=96 then q is −101 through 90).

Hence (for the example parameters) the smallest non-zero positive number that 
can be represented is 1×10^−101 and the largest is 9999999×10^90 (9.999999×10^96), 
and the full range of numbers is −9.999999×10^96 through 9.999999×10^96.

The numbers −b^(1−emax) and b^(1−emax) (here, −1×10^−95 and 1×10^−95) are the smallest 
(in magnitude) normal numbers; non-zero numbers between these smallest numbers 
are called *subnormal numbers*.

Zero values are finite values with significand 0. These are signed zeros, the 
sign bit specifies if a zero is +0 (positive zero) or −0 (negative zero).


## Representation and encoding in memory
Some numbers may have several representations in the model. 
For instance, if b=10 and p=7, −12.345 can be represented by −12345×10−3, 
−123450×10−4, and −1234500×10−5. However, for most operations, such as arithmetic 
operations, the result (value) does not depend on the representation of the inputs.

For the *decimal formats*, any representation is valid, and the set of these 
representations is called a *cohort*. When a result can have several 
representations, the standard specifies which member of the cohort is chosen.

For the *binary formats*, the representation is made unique by choosing the 
smallest representable exponent that retains the most significant bit (MSB) 
within the selected word size and format. 

Further, the exponent is not represented directly, but a bias is added so that 
the smallest representable exponent is represented as 1, with 0 used for 
*subnormal numbers*. For numbers with an exponent in the normal range (not all 
ones or all zeros), the leading bit of the significand will always be 1.

Consequently, a leading 1 can be implied rather than explicitly present in the 
memory encoding, and under the standard the explicitly represented part of the 
significand will lie between 0 and 1.

This rule is called *leading bit convention*, *implicit bit convention*, or 
*hidden bit convention*. The rule allows the memory format to have one more bit 
of precision. The leading bit convention is not used for the subnormal numbers: 
they have an exponent outside the normal exponent range and scale by the smallest 
represented exponent as used for the smallest normal numbers.

The smallest number representable has no 1 bit in the subnormal significand and 
is called the *positive or negative zero* as determined by the sign. It actually 
represents a *rounding to zero* of numbers in the range between zero and the 
smallest representable non-zero number of the same sign, which is why it has a 
sign, and why its reciprocal +Inf or -Inf also has a sign.


## Basic and interchange formats

The following table summarizes the smallest interchange formats:

name       cn        b   sb     dd  eb      decEmax  ExponentBias    E min   E max   Notes
binary16   Half      2   11   3.31   5         4.51     15=2^4−1       −14      15   not basic
`binary32  Single    2   24   7.22   8        38.23    127=2^7−1      −126     127`
`binary64  Double    2   53  15.95  11       307.95   1023=2^10−1    −1022    1023`
binary128  Quadruple 2  113  34.02  15      4931.77  16383=2^14−1   −16382   16383
binary256  Octuple   2  237  71.34  19     78913.2  262143=2^18−1  −262142  262143   not basic
decimal32           10    7   7      7.58     96       101             −95      96   not basic
decimal64           10   16  16      9.58    384       398            −383     384
decimal128          10   34  34     13.58   6144      6176           −6143    6144

b  = base
cn = common name (precision)
sb = significand bits (digits)
dd = decimal digits
eb = exponent bits
decEmax = decimal E max

Note that in the table above, the minimum exponents listed are for normal numbers; 
the special subnormal number representation allows even smaller numbers to be 
represented (with some loss of precision). 
For example, the smallest double-precision number greater than zero that can be 
represented in that form is 2^−1074 (because 1074 = 1022 + 53 − 1).

Decimal digits is `digits × log10 base`, this gives an approximate precision in decimal.
Decimal E max is `Emax × log10 base`, this gives the maximum exponent in decimal.

