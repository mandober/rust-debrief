fn rposition<P>(&mut self, predicate: P) -> Option<usize>
   where P: FnMut(Self::Item) -> bool,
         Self: ExactSizeIterator + DoubleEndedIterator;
/**
Searches for an element in an iterator
from the right, returning its index.

rposition() takes a closure that returns true or false.
It applies this closure to each element of the iterator,
starting from the end, and if one of them returns true,
then rposition() returns Some(index).
If all of them return false, it returns None.

rposition() is short-circuiting; in other words,
it will stop processing as soon as it finds a true.
*/

let a = [1, 2, 3];
assert_eq!(a.iter().rposition(|&x| x == 3), Some(2));
assert_eq!(a.iter().rposition(|&x| x == 5), None);

// Stopping at the first true:
let a = [1, 2, 3];
let mut iter = a.iter();
assert_eq!(iter.rposition(|&x| x == 2), Some(1));

// we can still use `iter`, as there are more elements.
assert_eq!(iter.next(), Some(&1));
