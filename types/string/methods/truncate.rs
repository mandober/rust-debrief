fn truncate(&mut self, new_len: usize)[src]
[−]

Shortens this String to the specified length.

If new_len is greater than the string's current length, this has no effect.

Note that this method has no effect on the allocated capacity of the string
Panics

Panics if new_len does not lie on a char boundary.
Examples

Basic usage:

let mut s = String::from("hello");

s.truncate(2);

assert_eq!("he", s);