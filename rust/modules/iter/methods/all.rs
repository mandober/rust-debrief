fn all<F>(&mut self, f: F) -> bool
  where
    F: FnMut(Self::Item) -> bool;
/**
Tests if every element of the iterator matches a predicate.

all() takes a closure that returns true or false.

It applies this closure to each element of the iterator,
and if they all return true, then so does all().
If any of them return false, it returns false.

all() is short-circuiting; in other words, it will stop processing as soon as
it finds a false, given that no matter what else happens,
the result will also be false.

An empty iterator returns true.

EXAMPLES:
*/
let a = [1, 2, 3];
assert!(a.iter().all(|&x| x > 0));
assert!(!a.iter().all(|&x| x > 2));

// Stopping at the first false:
let a = [1, 2, 3];
let mut iter = a.iter();
assert!(!iter.all(|&x| x != 2));

// we can still use `iter`, as there are more elements.
assert_eq!(iter.next(), Some(&3));
