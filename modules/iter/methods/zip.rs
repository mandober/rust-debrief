fn zip<U>(self, other: U) -> Zip<Self, <U as IntoIterator>::IntoIter>
   where U: IntoIterator;
/**
Zips up two iterators into a single iterator of pairs.

zip() returns a new iterator that will iterate over two other iterators,
returning a TUPLE where the first element comes from the first iterator,
and the second element comes from the second iterator.
In other words, it zips two iterators together, into a single one.
When either iterator returns None, all further calls to next will return None.

EXAMPLES:
*/
let a1 = [1, 2, 3];
let a2 = [4, 5, 6];
let mut iter = a1.iter().zip(a2.iter());
assert_eq!(iter.next(), Some((&1, &4)));
assert_eq!(iter.next(), Some((&2, &5)));
assert_eq!(iter.next(), Some((&3, &6)));
assert_eq!(iter.next(), None);

/**
Since the argument to zip() uses IntoIterator, we can pass anything that can be
converted into an Iterator, not just an Iterator itself. For example,
slices &[T] implement IntoIterator, so they can be passed to zip directly:
*/
let s1 = &[1, 2, 3];
let s2 = &[4, 5, 6];
let mut iter = s1.iter().zip(s2);
assert_eq!(iter.next(), Some((&1, &4)));
assert_eq!(iter.next(), Some((&2, &5)));
assert_eq!(iter.next(), Some((&3, &6)));
assert_eq!(iter.next(), None);

/**
zip is often used to zip an infinite iterator to a finite one.
This works because the finite iterator will eventually return None,
ending the zipper. Zipping with (0..) can look a lot like enumerate:
*/
let enumerate: Vec<_> = "foo".chars().enumerate().collect();
let zipper: Vec<_> = (0..).zip("foo".chars()).collect();

assert_eq!((0, 'f'), enumerate[0]);
assert_eq!((0, 'f'), zipper[0]);

assert_eq!((1, 'o'), enumerate[1]);
assert_eq!((1, 'o'), zipper[1]);

assert_eq!((2, 'o'), enumerate[2]);
assert_eq!((2, 'o'), zipper[2]);
