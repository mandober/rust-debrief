fn with_capacity(capacity: usize) -> String[src]
[−]

Creates a new empty String with a particular capacity.

Strings have an internal buffer to hold their data. The capacity is the length of that buffer, and can be queried with the capacity method. This method creates an empty String, but one with an initial buffer that can hold capacity bytes. This is useful when you may be appending a bunch of data to the String, reducing the number of reallocations it needs to do.

If the given capacity is 0, no allocation will occur, and this method is identical to the new method.
Examples

Basic usage:

let mut s = String::with_capacity(10);

// The String contains no chars, even though it has capacity for more
assert_eq!(s.len(), 0);

// These are all done without reallocating...
let cap = s.capacity();
for i in 0..10 {
    s.push('a');
}

assert_eq!(s.capacity(), cap);

// ...but this may make the vector reallocate
s.push('a');