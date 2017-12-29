fn nth(&mut self, n: usize) -> Option<Self::Item>;
/**
Returns the nth element of the iterator.

Like most indexing operations, the count starts from zero, so
nth(0) returns the first value, nth(1) the second, and so on.

Note that all preceding elements, as well as the returned element, will be
consumed from the iterator. That means that the preceding elements will be
discarded, and also that calling nth(0) multiple times on the same iterator
will return different elements.

nth() will return None if n is greater than
or equal to the length of the iterator.
*/
let a = [1, 2, 3];
assert_eq!(a.iter().nth(1), Some(&2));

// Calling nth() multiple times doesn't rewind the iterator:
let a = [1, 2, 3];
let mut iter = a.iter();
assert_eq!(iter.nth(1), Some(&2));
assert_eq!(iter.nth(1), None);

// Returning None if there are less than n + 1 elements:
let a = [1, 2, 3];
assert_eq!(a.iter().nth(10), None);
