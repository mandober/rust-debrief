# String slice

A string slice, `&str` is a reference to the part of a string.

Slice is a *2-part fat pointer*:
a *pointer* to the string, and the *length* of the slice.

String slice has *no ownership* over data it points to, it borrows it.

String slices are either mutable or shared:
- shared slice `&str`
- mutable slice `&mut str`

A value of type `str` is a Unicode string, represented as an array of 8-bit
unsigned bytes holding a sequence of UTF-8 code points. Since `str` is a
dynamically sized type, it is *not a first-class* type, but can only be
instantiated through a pointer type, such as `&str`.

The type str in Rust is special: it is primitive, so that the compiler can 
optimize certain string operations; but it is not first class, so it is not 
possible to define variables of type str or pass str values directly to functions. 
To use Rust strings in programs, one should use string references.
