fn find<P>(&mut self, predicate: P) -> Option<Self::Item>
   where P: FnMut(&Self::Item) -> bool;
/**
Searches for an element of an iterator that satisfies a predicate.

find() takes a closure that returns true or false.

It applies this closure to each element of the iterator, and
if any of them return true, then find() returns Some(element).

If they all return false, it returns None.

find() is short-circuiting; in other words, it will
stop processing as soon as the closure returns true.

Because find() takes a reference, and many iterators iterate over references,
this leads to a possibly confusing situation where the argument is a double
reference. You can see this effect in the examples below, with &&x.

EXAMPLES:
*/
let a = [1, 2, 3];
assert_eq!(a.iter().find(|&&x| x == 2), Some(&2));
assert_eq!(a.iter().find(|&&x| x == 5), None);

// Stopping at the first true:
let a = [1, 2, 3];
let mut iter = a.iter();
assert_eq!(iter.find(|&&x| x == 2), Some(&2));

// we can still use `iter`, as there are more elements.
assert_eq!(iter.next(), Some(&3));
