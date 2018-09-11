fn position<P>(&mut self, predicate: P) -> Option<usize>
   where P: FnMut(Self::Item) -> bool;
/**
Searches for an element in an iterator, returning its index.

position() takes a closure that returns true or false.
It applies this closure to each element of the iterator,
and if one of them returns true, then position() returns Some(index).
If all of them return false, it returns None.

position() is short-circuiting; in other words,
it will stop processing as soon as it finds a true.

Overflow Behavior:
The method does no guarding against overflows,
so if there are more than usize::MAX non-matching elements,
it either produces the wrong result or panics.
If debug assertions are enabled, a panic is guaranteed.

Panics:
This function might panic if the iterator has
more than usize::MAX non-matching elements.
*/

let a = [1, 2, 3];
assert_eq!(a.iter().position(|&x| x == 2), Some(1));
assert_eq!(a.iter().position(|&x| x == 5), None);

// Stopping at the first true:
let a = [1, 2, 3, 4];
let mut iter = a.iter();
assert_eq!(iter.position(|&x| x >= 2), Some(1));

// we can still use `iter`, as there are more elements.
assert_eq!(iter.next(), Some(&3));

// The returned index depends on iterator state
assert_eq!(iter.position(|&x| x == 4), Some(0));
