# Machine architecture dependent integers

- type: primitive, scalar, concrete, fixed, sized
- Machine architecture dependent integers represent platfom's pointer size.
- max`usize` value represents max memory address in the process' space.
- max`isize` value represents theoretical upper bound on an object's size.


## usize and isize
Machine architecture dependent integers represent memory address. They come in two types: as signed `isize` and unsigned `usize`. Both types have the same number of bits as the __platform's pointer type__. On 64-bit architecture (e.g. x86_64) it holds 64 bits (8 Bytes) and on 32-bit arch (e.g. x86) it is 32 bits (4 Bytes).

## usize
`usize` is the unsigned integer type with the same number of bits as the platform's pointer type. It can represent __every memory address__ in the process' space.

On a 64-bit platform, the bit pattern used to store `usize` and `u64` is the same, alike to 32-bit platform's storage of `usize` and `u32`. Nevertheless, to represent anything having to do with (an unknown) size (lenght of sequence, index of element, etc.) `usize` type is always used.

## isize
`isize` is the signed integer type. The maximum `isize` value represents __theoretical upper bound__ on an object's size. This ensures that `isize` can be used to calculate differences between pointers and can address every byte within an object along with one byte past its end. `isize` complements `usize` like other integer types do, but its practical use is to represent a difference between sizes, since this can yield a negative number (subtraction between two memory addresses, two indexes of an array, etc.).
