fn chain<U>(self, other: U) -> Chain<Self, <U as IntoIterator>::IntoIter>
  where
    U: IntoIterator<Item = Self::Item>;
/**
Takes two iterators and creates a new iterator over both in sequence.

chain() will return a new iterator which will first iterate over values from
the first iterator and then over values from the second iterator.

In other words, it links two iterators together, in a chain. 🔗

EXAMPLES:
*/
let a1 = [1, 2, 3];
let a2 = [4, 5, 6];
let mut iter = a1.iter().chain(a2.iter());
assert_eq!(iter.next(), Some(&1));
assert_eq!(iter.next(), Some(&2));
assert_eq!(iter.next(), Some(&3));
assert_eq!(iter.next(), Some(&4));
assert_eq!(iter.next(), Some(&5));
assert_eq!(iter.next(), Some(&6));
assert_eq!(iter.next(), None);

/**
Since the argument to chain() uses IntoIterator, we can pass anything that can
be converted into an Iterator, not just an Iterator itself. For example,
slices (&[T]) implement IntoIterator, and so can be passed to chain() directly:
*/
let s1 = &[1, 2, 3];
let s2 = &[4, 5, 6];
let mut iter = s1.iter().chain(s2);
assert_eq!(iter.next(), Some(&1));
assert_eq!(iter.next(), Some(&2));
assert_eq!(iter.next(), Some(&3));
assert_eq!(iter.next(), Some(&4));
assert_eq!(iter.next(), Some(&5));
assert_eq!(iter.next(), Some(&6));
assert_eq!(iter.next(), None);
