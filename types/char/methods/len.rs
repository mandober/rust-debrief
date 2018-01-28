fn len_utf8(self) -> usize[src]
[−]

Returns the number of bytes this char would need if encoded in UTF-8.

That number of bytes is always between 1 and 4, inclusive.
Examples

Basic usage:

let len = 'A'.len_utf8();
assert_eq!(len, 1);

let len = 'ß'.len_utf8();
assert_eq!(len, 2);

let len = 'ℝ'.len_utf8();
assert_eq!(len, 3);

let len = '💣'.len_utf8();
assert_eq!(len, 4);
Run

The &str type guarantees that its contents are UTF-8, and so we can compare the length it would take if each code point was represented as a char vs in the &str itself:

// as chars
let eastern = '東';
let capitol = '京';

// both can be represented as three bytes
assert_eq!(3, eastern.len_utf8());
assert_eq!(3, capitol.len_utf8());

// as a &str, these two are encoded in UTF-8
let tokyo = "東京";

let len = eastern.len_utf8() + capitol.len_utf8();

// we can see that they take six bytes total...
assert_eq!(6, tokyo.len());

// ... just like the &str
assert_eq!(len, tokyo.len());
Run

fn len_utf16(self) -> usize[src]
[−]

Returns the number of 16-bit code units this char would need if encoded in UTF-16.

See the documentation for len_utf8 for more explanation of this concept. This function is a mirror, but for UTF-16 instead of UTF-8.
Examples

Basic usage:

let n = 'ß'.len_utf16();
assert_eq!(n, 1);

let len = '💣'.len_utf16();
assert_eq!(len, 2);