fn reserve(&mut self, additional: usize)[src]
[−]

Ensures that this String's capacity is at least additional bytes larger than its length.

The capacity may be increased by more than additional bytes if it chooses, to prevent frequent reallocations.

If you do not want this "at least" behavior, see the reserve_exact method.
Panics

Panics if the new capacity overflows usize.
Examples

Basic usage:

let mut s = String::new();

s.reserve(10);

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
s.reserve(8);

// ... doesn't actually increase.
assert_eq!(10, s.capacity());