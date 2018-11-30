# Fixed-point numbers

https://en.wikipedia.org/wiki/Fixed-point_arithmetic

In fixed-point systems, the position of radix point in the sequence of digits is fixed.

So a fixed-point scheme might be to use a sequence of 8 decimal digits with the decimal point in the middle, whereby "00012345" would represent 0001.2345.

A fixed-point number representation is a real data type for a number that has a fixed number of digits after (and sometimes also before) the radix point. Fixed-point number representation can be compared to the more complicated (and more computationally demanding) floating-point number representation.

Fixed-point numbers are useful for representing fractional values, usually in base 2 or base 10, when the executing processor has no FPU or if fixed-point provides improved performance or accuracy for the application at hand.

A value of a fixed-point data type is essentially an integer that is scaled by an implicit specific factor determined by the type. Unlike floating-point data types, the scaling factor is the same for all values of the same type, and does not change during the entire computation. For example, the value 1.23 can be represented as 1230 in a fixed-point data type with scaling factor of 1/1000, and the value 1,230,000 can be represented as 1230 with a scaling factor of 1000. The scaling factor is usually a power of 10 (for human convenience) or a power of 2 (for computational efficiency).

The maximum value of a fixed-point type is simply the largest value that can be represented in the _underlying integer type_ multiplied by the scaling factor; and similarly for the minimum value.

Because fixed point operations can produce results that overflow, information loss is possible. For instance, to store the result of fixed point multiplication could potentially require much more bits than could fit in either operand. In order to fit the result into the same number of bits as the operands, the answer must be rounded or truncated. With $$I$$ integer bits, and $$Q$$ fractional bits, the answer could have up to $$2I$$ integer bits, and $$2Q$$ fractional bits.

For simplicity, many fixed-point multiply procedures use the same result format as the operands. This has the effect of keeping the middle bits; the $$I$$-number of least significant integer bits, and the $$Q$$-number of most significant fractional bits.

Fractional bits lost below this value represent a precision loss which is common in fractional multiplication. If any integer bits are lost, however, the value will be radically inaccurate. Division frequantly uses, also highly inaccurate, saturation.

Very few programming languages include built-in support for fixed point values, because for most applications, floating-point representations are usually simpler to use and accurate enough. Floating-point representations are easier to use than fixed-point representations, because they can handle a wider dynamic range and do not require programmers to specify the number of digits after the radix point. However, if they are really needed, it is possible to implement fixed-point numbers in any language.

