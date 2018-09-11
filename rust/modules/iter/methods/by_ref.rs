fn by_ref(&mut self) -> &mut Self;
/**
Borrows an iterator, rather than consuming it.

This is useful to allow applying iterator adaptors
while still retaining ownership of the original iterator.

EXAMPLES:
*/
let a = [1, 2, 3];
let iter = a.into_iter();
let sum: i32 = iter.take(5).fold(0, |acc, &i| acc + i );
assert_eq!(sum, 6);

// if we try to use iter again, it won't work.
// The following line gives:
//   "error: use of moved value: `iter`
//   assert_eq!(iter.next(), None);

// let's try that again
let a = [1, 2, 3];
let mut iter = a.into_iter();
// instead, we add in a by_ref()
let sum: i32 = iter.by_ref()
                   .take(2)
                   .fold(0, |acc, &i| acc + i );
assert_eq!(sum, 3);

// now this is just fine:
assert_eq!(iter.next(), Some(&3));
assert_eq!(iter.next(), None);
