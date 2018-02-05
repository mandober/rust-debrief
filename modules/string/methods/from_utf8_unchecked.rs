unsafe fn from_utf8_unchecked(bytes: Vec<u8>) -> String[src]
[−]

Converts a vector of bytes to a String without checking that the string contains valid UTF-8.

See the safe version, from_utf8, for more details.
Safety

This function is unsafe because it does not check that the bytes passed to it are valid UTF-8. If this constraint is violated, it may cause memory unsafety issues with future users of the String, as the rest of the standard library assumes that Strings are valid UTF-8.
Examples

Basic usage:

// some bytes, in a vector
let sparkle_heart = vec![240, 159, 146, 150];

let sparkle_heart = unsafe {
    String::from_utf8_unchecked(sparkle_heart)
};

assert_eq!("💖", sparkle_heart);