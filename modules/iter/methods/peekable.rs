fn peekable(self) -> Peekable<Self>;

/**
Creates an iterator which can use peek to look at
the next element of the iterator without consuming it.

Adds a peek method to an iterator.

Note that the underlying iterator is still
advanced when peek is called for the first time:
  In order to retrieve the next element,
  next is called on the underlying iterator,
  hence any side effects
  (i.e. anything other than fetching the next value)
  of the next method will occur.

*/

let xs = [1, 2, 3];

let mut iter = xs.iter().peekable();

// peek() lets us see into the future
assert_eq!(iter.peek(), Some(&&1));
assert_eq!(iter.next(), Some(&1));
assert_eq!(iter.next(), Some(&2));

// we can peek() multiple times, the iterator won't advance
assert_eq!(iter.peek(), Some(&&3));
assert_eq!(iter.peek(), Some(&&3));
assert_eq!(iter.next(), Some(&3));

// after the iterator is finished, so is peek()
assert_eq!(iter.peek(), None);
assert_eq!(iter.next(), None);
