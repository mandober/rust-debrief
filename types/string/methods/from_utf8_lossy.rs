fn from_utf8_lossy(v: &'a [u8]) -> Cow<'a, str>[src]
[−]

Converts a slice of bytes to a string, including invalid characters.

Strings are made of bytes (u8), and a slice of bytes (&[u8]) is made of bytes, so this function converts between the two. Not all byte slices are valid strings, however: strings are required to be valid UTF-8. During this conversion, from_utf8_lossy() will replace any invalid UTF-8 sequences with U+FFFD REPLACEMENT CHARACTER, which looks like this: �

If you are sure that the byte slice is valid UTF-8, and you don't want to incur the overhead of the conversion, there is an unsafe version of this function, from_utf8_unchecked, which has the same behavior but skips the checks.

This function returns a Cow<'a, str>. If our byte slice is invalid UTF-8, then we need to insert the replacement characters, which will change the size of the string, and hence, require a String. But if it's already valid UTF-8, we don't need a new allocation. This return type allows us to handle both cases.
Examples

Basic usage:

// some bytes, in a vector
let sparkle_heart = vec![240, 159, 146, 150];

let sparkle_heart = String::from_utf8_lossy(&sparkle_heart);

assert_eq!("💖", sparkle_heart);
Run

Incorrect bytes:

// some invalid bytes
let input = b"Hello \xF0\x90\x80World";
let output = String::from_utf8_lossy(input);

assert_eq!("Hello �World", output);