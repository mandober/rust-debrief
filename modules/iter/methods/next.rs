fn next(&mut self) -> Option<Self::Item>;
/**
Advances the iterator and returns the next value.

Returns None when iteration is finished.

Individual iterator implementations may choose to resume iteration, and so
calling next() again may or may not eventually start returning Some(Item)
again at some point.

*/
let a = [1, 2, 3];
let mut iter = a.iter();

// A call to next() returns the next value...
assert_eq!(Some(&1), iter.next());
assert_eq!(Some(&2), iter.next());
assert_eq!(Some(&3), iter.next());

// ... and then None once it's over.
assert_eq!(None, iter.next());

// More calls may or may not return None. Here, they always will.
assert_eq!(None, iter.next());
assert_eq!(None, iter.next());
