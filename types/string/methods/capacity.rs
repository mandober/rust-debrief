fn capacity(&self) -> usize[src]
[−]

Returns this String's capacity, in bytes.
Examples

Basic usage:

let s = String::with_capacity(10);

assert!(s.capacity() >= 10);