fn rev(self) -> Rev<Self>
   where Self: DoubleEndedIterator;
/**
Reverses an iterator's direction.

Usually, iterators iterate from left to right.

After using rev(), an iterator will instead iterate from right to left.

This is only possible if the iterator has an end,
so rev() only works on DoubleEndedIterators.

*/
let a = [1, 2, 3];

let mut iter = a.iter().rev();

assert_eq!(iter.next(), Some(&3));
assert_eq!(iter.next(), Some(&2));
assert_eq!(iter.next(), Some(&1));

assert_eq!(iter.next(), None);
