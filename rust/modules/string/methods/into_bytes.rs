fn into_bytes(self) -> Vec<u8>[src]
[−]

Converts a String into a byte vector.

This consumes the String, so we do not need to copy its contents.
Examples

Basic usage:

let s = String::from("hello");
let bytes = s.into_bytes();

assert_eq!(&[104, 101, 108, 108, 111][..], &bytes[..]);