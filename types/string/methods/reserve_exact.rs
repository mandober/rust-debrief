fn reserve_exact(&mut self, additional: usize)[src]
[−]

Ensures that this String's capacity is additional bytes larger than its length.

Consider using the reserve method unless you absolutely know better than the allocator.
Panics

Panics if the new capacity overflows usize.
Examples

Basic usage:

let mut s = String::new();

s.reserve_exact(10);

assert!(s.capacity() >= 10);
Run

This may not actually increase the capacity:

let mut s = String::with_capacity(10);
s.push('a');
s.push('b');

// s now has a length of 2 and a capacity of 10
assert_eq!(2, s.len());
assert_eq!(10, s.capacity());

// Since we already have an extra 8 capacity, calling this...
s.reserve_exact(8);

// ... doesn't actually increase.
assert_eq!(10, s.capacity());