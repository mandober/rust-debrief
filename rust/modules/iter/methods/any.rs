fn any<F>(&mut self, f: F) -> bool
   where  F: FnMut(Self::Item) -> bool;
/**
Tests if any element of the iterator matches a predicate.

any() takes a closure that returns true or false.
It applies this closure to each element of the iterator,
and if any of them return true, then so does any().
If they all return false, it returns false.

any() is short-circuiting; in other words,
it will stop processing as soon as it finds a true,
given that no matter what else happens, the result will also be true.

An empty iterator returns false.

EXAMPLES:
*/
let a = [1, 2, 3];
assert!(a.iter().any(|&x| x > 0));
assert!(!a.iter().any(|&x| x > 5));

// Stopping at the first true:
let a = [1, 2, 3];
let mut iter = a.iter();
assert!(iter.any(|&x| x != 2));

// we can still use `iter`, as there are more elements.
assert_eq!(iter.next(), Some(&2));
