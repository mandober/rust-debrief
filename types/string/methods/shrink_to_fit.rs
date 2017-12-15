fn shrink_to_fit(&mut self)[src]
[−]

Shrinks the capacity of this String to match its length.
Examples

Basic usage:

let mut s = String::from("foo");

s.reserve(100);
assert!(s.capacity() >= 100);

s.shrink_to_fit();
assert_eq!(3, s.capacity());