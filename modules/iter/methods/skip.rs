fn skip(self, n: usize) -> Skip<Self>;
/**
Creates an iterator that skips the first n elements.

After they have been consumed, the rest of the elements are yielded.
*/

let a = [1, 2, 3];

let mut iter = a.iter().skip(2);

assert_eq!(iter.next(), Some(&3));
assert_eq!(iter.next(), None);
