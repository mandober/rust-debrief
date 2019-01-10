fn as_bytes(&self) -> &[u8]

// Returns a byte slice of this String's contents.
// The inverse of this method is from_utf8.

let s = String::from("hello");
assert_eq!(&[104, 101, 108, 108, 111], s.as_bytes());
