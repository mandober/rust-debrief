fn from_utf8(vec: Vec<u8>) -> Result<String, FromUtf8Error>;
/**
Converts a vector of bytes to a String.

A string slice (&str) is made of bytes (u8), and a vector of bytes (Vec<u8>) is made of bytes, so this function converts between the two. Not all byte slices are valid Strings, however: String requires that it is valid UTF-8. from_utf8() checks to ensure that the bytes are valid UTF-8, and then does the conversion.

If you are sure that the byte slice is valid UTF-8, and you don't want to incur the overhead of the validity check, there is an unsafe version of this function, from_utf8_unchecked, which has the same behavior but skips the check.

This method will take care to not copy the vector, for efficiency's sake.

If you need a &str instead of a String, consider str::from_utf8.

The inverse of this method is as_bytes.
Errors

Returns Err if the slice is not UTF-8 with a description as to why the provided bytes are not UTF-8. The vector you moved in is also included.
Examples

Basic usage:
*/

// some bytes, in a vector
let sparkle_heart = vec![240, 159, 146, 150];

// We know these bytes are valid, so we'll use `unwrap()`.
let sparkle_heart = String::from_utf8(sparkle_heart).unwrap();

assert_eq!("💖", sparkle_heart);
Run

Incorrect bytes:

// some invalid bytes, in a vector
let sparkle_heart = vec![0, 159, 146, 150];

assert!(String::from_utf8(sparkle_heart).is_err());
Run

See the docs for FromUtf8Error for more details on what you can do with this error.
