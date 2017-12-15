fn push_str(&mut self, string: &str)[src]
[−]

Appends a given string slice onto the end of this String.
Examples

Basic usage:

let mut s = String::from("foo");

s.push_str("bar");

assert_eq!("foobar", s);