//! Collection types


/**
The char type represents a single UNICODE SCALAR VALUE.

- More specifically, since character isn't a well-defined concept in Unicode,
  char is a UNICODE SCALAR VALUE, which is similar to, but not the same as,
  a UNICODE CODE POINT.
- UNICODE SCALAR VALUES: from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive.
- char is always 4 bytes in size (4b)
- create chars always in single quotes

METHODS:
is_alphabetic()
is_numeric()
is_whitespace()
is_lowercase()
*/

let c = 'x';
let hearts = '💕';

if c.is_alphabetic() {
    println!("Alphabetical");
} else if c.is_numeric() {
    println!("Numerical");
} else {
    println!("Neither");
}
